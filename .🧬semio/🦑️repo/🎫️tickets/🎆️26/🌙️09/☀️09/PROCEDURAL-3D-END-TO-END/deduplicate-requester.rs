//! 🧊 Folds byte-identical direct-call targets and preserves all observable function identities.
use std::{collections::{HashMap, HashSet}, convert::Infallible, env, fs, path::Path};
use wasm_encoder::{reencode::{utils, Error, Reencode}, CodeSection, FunctionSection, Module};
use wasmparser::{ElementItems, ExternalKind, FunctionBody, Operator, OperatorsReader, Parser, Payload, TableInit, TypeRef};

fn address_uses(mut reader: OperatorsReader<'_>, protected: &mut HashSet<u32>) -> Result<(), wasmparser::BinaryReaderError> {
    while !reader.eof() {
        if let Operator::RefFunc { function_index } = reader.read()? { protected.insert(function_index); }
    }
    Ok(())
}

struct Fold { map: Vec<u32>, keep: Vec<bool>, body: usize }
impl Reencode for Fold {
    type Error = Infallible;
    fn function_index(&mut self, index: u32) -> Result<u32, Error<Infallible>> { Ok(self.map[index as usize]) }
    fn parse_function_section(&mut self, out: &mut FunctionSection, input: wasmparser::FunctionSectionReader<'_>) -> Result<(), Error<Infallible>> {
        for (i, ty) in input.into_iter().enumerate() { let ty=ty?; if self.keep[i] { out.function(ty); } }
        Ok(())
    }
    fn parse_function_body(&mut self, out: &mut CodeSection, body: FunctionBody<'_>) -> Result<(), Error<Infallible>> {
        self.body += 1;
        if self.keep[self.body-1] { utils::parse_function_body(self, out, body)?; }
        Ok(())
    }
    fn parse_custom_section(&mut self, out: &mut Module, section: wasmparser::CustomSectionReader<'_>) -> Result<(), Error<Infallible>> {
        if section.name() != "name" { out.section(&self.custom_section(section)?); }
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().collect();
    let bytes = fs::read(&args[1])?;
    let mut imports = 0u32;
    let mut types = vec![];
    let mut bodies = vec![];
    let mut protected = HashSet::new();
    for payload in Parser::new(0).parse_all(&bytes) {
        match payload? {
            Payload::ImportSection(s) => for entry in s.into_imports() { if matches!(entry?.ty, TypeRef::Func(_) | TypeRef::FuncExact(_)) { imports+=1; } },
            Payload::FunctionSection(s) => types = s.into_iter().collect::<Result<Vec<_>,_>>()?,
            Payload::ExportSection(s) => for entry in s { let e=entry?; if e.kind==ExternalKind::Func { protected.insert(e.index); } },
            Payload::StartSection{func,..} => { protected.insert(func); },
            Payload::GlobalSection(s) => for entry in s { address_uses(entry?.init_expr.get_operators_reader(), &mut protected)?; },
            Payload::TableSection(s) => for entry in s { if let TableInit::Expr(e) = entry?.init { address_uses(e.get_operators_reader(), &mut protected)?; } },
            Payload::ElementSection(s) => for entry in s { match entry?.items {
                ElementItems::Functions(list) => for f in list { protected.insert(f?); },
                ElementItems::Expressions(_,list) => for e in list { address_uses(e?.get_operators_reader(), &mut protected)?; },
            } },
            Payload::CodeSectionEntry(body) => {
                let raw=&bytes[body.range()];
                if raw.contains(&0xd2) { address_uses(body.get_operators_reader()?, &mut protected)?; }
                bodies.push(raw);
            }
            _ => {}
        }
    }
    assert_eq!(types.len(), bodies.len());
    let mut map: Vec<_> = (0..imports).collect();
    let mut keep = Vec::with_capacity(types.len());
    let mut seen = HashMap::new();
    let mut next = imports;
    for (i, (ty, body)) in types.into_iter().zip(bodies).enumerate() {
        let candidate = (ty,body);
        let prior = if protected.contains(&(i as u32+imports)) { None } else { seen.get(&candidate).copied() };
        keep.push(prior.is_none());
        if let Some(old) = prior { map.push(old); } else {
            map.push(next);
            seen.entry(candidate).or_insert(next);
            next+=1;
        }
    }
    eprintln!("[DEBUG] Direct-call fold retains {next} total functions of {} including {imports} imports",map.len());
    drop(seen);
    if args.get(3).is_some_and(|arg| arg == "--index") {
        let mut reverse = Vec::with_capacity(4*next as usize);
        for old in 0..map.len() {
            if old < imports as usize || keep[old - imports as usize] {
                assert_eq!(map[old] as usize, reverse.len()/4);
                reverse.extend_from_slice(&(old as u32).to_le_bytes());
            }
        }
        fs::write(&args[2], reverse)?;
        return Ok(());
    }
    if next as usize == map.len() { return Err("no duplicate direct-call targets remain".into()); }
    let mut fold = Fold{map,keep,body:0};
    let mut module=Module::new();
    fold.parse_core_module(&mut module, Parser::new(0), &bytes)?;
    let target=Path::new(&args[2]);
    let pending=target.with_extension("writing.wasm");
    fs::write(&pending,module.finish())?;
    fs::rename(pending,target)?;
    Ok(())
}
