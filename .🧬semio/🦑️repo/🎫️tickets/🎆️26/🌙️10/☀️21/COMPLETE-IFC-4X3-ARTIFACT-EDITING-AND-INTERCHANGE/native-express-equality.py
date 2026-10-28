from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/evaluator.rs');s=p.read_text(encoding='utf-8');a=s.index('                    let mut remaining = Vec::new();');b=s.index('                    result\n',a)+len('                    result\n');s=s[:a]+'''                    if unordered {if self.bag_cover(a,b,false,identity,depth+1)?{True}else if self.bag_cover(a,b,true,identity,depth+1)?{Indeterminate}else{False}}
                    else {let mut result=True;for(index,value)in a.iter().enumerate(){match self.equal_inner(value,&b[index],identity,depth+1)?{False=>return Ok(False),Indeterminate=>result=Indeterminate,_=>{}}}result}
'''+s[b:]
s=s.replace('potential: bool, seen: &mut [bool], matched: &mut [Option<usize>], depth: usize','potential: bool, identity:bool,equality_depth:usize,seen: &mut [bool], matched: &mut [Option<usize>], depth: usize')
s=s.replace('self.equal_inner(&left[index], &right[candidate], true, 0)?','self.equal_inner(&left[index], &right[candidate], identity, equality_depth)?')
s=s.replace('self.bag_augment(left, right, matched[candidate].unwrap(), potential, seen, matched, depth + 1)?','self.bag_augment(left, right, matched[candidate].unwrap(), potential,identity,equality_depth,seen, matched, depth + 1)?')
s=s.replace('potential: bool) -> ExpressResult<bool>','potential: bool,identity:bool,equality_depth:usize) -> ExpressResult<bool>')
s=s.replace('self.bag_augment(left, right, index, potential, &mut seen, &mut matched, 0)?','self.bag_augment(left, right, index, potential,identity,equality_depth,&mut seen, &mut matched, 0)?')
s=s.replace('self.bag_cover(a, b, false)?','self.bag_cover(a,b,false,true,0)?').replace('self.bag_cover(a, b, true)?','self.bag_cover(a,b,true,true,0)?')
p.write_text(s,encoding='utf-8',newline='\n')