//! 📦️ Borrowed contributed target admission preserves exact laws without artifact dependencies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProviderRefusal { MissingDeclaration, InvalidDeclaration, Cancelled, Budget }
pub trait ProviderOperationControl { fn checkpoint(&mut self, completed: usize) -> Result<(), ProviderRefusal>; }
pub struct ProviderTargetInput<'a> { pub keys: &'a [&'a str], pub kind: Option<&'a str>, pub name: Option<&'a str>, pub laws: Option<&'a [&'a str]>, pub features: Option<&'a [&'a str]>, pub default_features: Option<bool> }
pub struct ProviderInput<'a> { pub keys: &'a [&'a str], pub schema_version: Option<u64>, pub targets: Option<&'a [ProviderTargetInput<'a>]> }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProviderTargetKind { Library, Test, Binary }
#[derive(Clone, Copy, Debug)]
pub struct ProviderTarget<'a> { pub kind: ProviderTargetKind, pub name: &'a str, pub laws: &'a [&'a str], pub features: &'a [&'a str], pub default_features: bool }
pub struct ProviderAdmission { completed: usize, admitted: usize }
impl Default for ProviderAdmission { fn default() -> Self { Self { completed: 0, admitted: 0 } } }
impl ProviderAdmission {
    pub fn completed(&self) -> usize { self.completed }
    pub fn admitted(&self) -> usize { self.admitted }
    fn step<C: ProviderOperationControl>(&mut self, control: &mut C) -> Result<(), ProviderRefusal> { control.checkpoint(self.completed)?; self.completed = self.completed.checked_add(1).ok_or(ProviderRefusal::Budget)?; Ok(()) }
    fn keys<C: ProviderOperationControl>(&mut self, keys: &[&str], expected: &[&str], control: &mut C) -> Result<(), ProviderRefusal> {
        if keys.len() != expected.len() { return Err(ProviderRefusal::InvalidDeclaration); }
        for (index, key) in keys.iter().enumerate() { self.step(control)?; if !expected.contains(key) || keys[..index].contains(key) { return Err(ProviderRefusal::InvalidDeclaration); } }
        Ok(())
    }
    fn text<C: ProviderOperationControl>(&mut self, text: &str, selector: u8, control: &mut C) -> Result<(), ProviderRefusal> {
        if text.is_empty() { return Err(ProviderRefusal::InvalidDeclaration); }
        let mut initial = true;
        let mut colon = false;
        for byte in text.bytes() {
            self.step(control)?;
            if selector == 0 { continue; }
            if selector == 2 { if !byte.is_ascii_alphanumeric() && byte != b'_' && byte != b'-' { return Err(ProviderRefusal::InvalidDeclaration); } continue; }
            if byte == b':' { if colon { colon = false; initial = true; } else { if initial { return Err(ProviderRefusal::InvalidDeclaration); } colon = true; } continue; }
            if colon || !(byte.is_ascii_alphabetic() || byte == b'_' || (!initial && byte.is_ascii_digit())) { return Err(ProviderRefusal::InvalidDeclaration); }
            initial = false;
        }
        if selector == 1 && (initial || colon) { return Err(ProviderRefusal::InvalidDeclaration); }
        Ok(())
    }
    fn strings<C: ProviderOperationControl>(&mut self, strings: &[&str], nonempty: bool, selector: u8, control: &mut C) -> Result<(), ProviderRefusal> {
        if nonempty && strings.is_empty() { return Err(ProviderRefusal::InvalidDeclaration); }
        for (index, string) in strings.iter().enumerate() {
            self.text(string, selector, control)?;
            for previous in &strings[..index] {
                self.step(control)?;
                if previous.len() != string.len() { continue; }
                let mut same = true;
                for (left, right) in previous.bytes().zip(string.bytes()) { self.step(control)?; if left != right { same = false; break; } }
                if same { return Err(ProviderRefusal::InvalidDeclaration); }
            }
        }
        Ok(())
    }
    pub fn admit<'a, C: ProviderOperationControl, R: FnMut(ProviderTarget<'a>)>(&mut self, input: Option<&'a ProviderInput<'a>>, control: &mut C, receive: &mut R) -> Result<(), ProviderRefusal> {
        self.step(control)?;
        let input = input.ok_or(ProviderRefusal::MissingDeclaration)?;
        self.keys(input.keys, &["schema-version", "law-targets"], control)?;
        if input.schema_version != Some(1) { return Err(ProviderRefusal::InvalidDeclaration); }
        let targets = input.targets.ok_or(ProviderRefusal::InvalidDeclaration)?;
        if targets.is_empty() || self.admitted != 0 { return Err(ProviderRefusal::InvalidDeclaration); }
        for target in targets {
            self.step(control)?;
            self.keys(target.keys, &["target-kind", "target-name", "laws", "features", "default-features"], control)?;
            let kind = match target.kind { Some("lib") => ProviderTargetKind::Library, Some("test") => ProviderTargetKind::Test, Some("bin") => ProviderTargetKind::Binary, _ => return Err(ProviderRefusal::InvalidDeclaration) };
            let name = target.name.ok_or(ProviderRefusal::InvalidDeclaration)?;
            self.text(name, 0, control)?;
            let laws = target.laws.ok_or(ProviderRefusal::InvalidDeclaration)?;
            self.strings(laws, true, 1, control)?;
            let features = target.features.ok_or(ProviderRefusal::InvalidDeclaration)?;
            self.strings(features, false, 2, control)?;
            let default_features = target.default_features.ok_or(ProviderRefusal::InvalidDeclaration)?;
            self.step(control)?;
            receive(ProviderTarget { kind, name, laws, features, default_features }); self.admitted += 1;
        }
        Ok(())
    }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
