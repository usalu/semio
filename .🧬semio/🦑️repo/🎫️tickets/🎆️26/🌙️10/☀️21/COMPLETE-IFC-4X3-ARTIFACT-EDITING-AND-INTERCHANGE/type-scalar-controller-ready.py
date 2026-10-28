from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-scalar-tests.rs');s=p.read_text(encoding='utf8');a='        let count = row["required"].as_array().unwrap().len();';assert s.count(a)==1;s=s.replace(a,'''        let Component::Extension(status)=&tree.root.children.get(0).unwrap().component else{panic!("missing typed status")};assert_eq!(serde_json::to_value(&status.props).unwrap()["ready"],true);
'''+a);data=s.encode();old=p.stat().st_size
with p.open('r+b')as output:output.write(data);output.write(b' '*max(0,old-len(data)))
p=Path(__file__).with_name('host-type-scalar-input-plan.md');s=p.read_text(encoding='utf8').replace('IfcExternallyDefinedTextFont.Name (string including explicit empty and multibyte text)','IfcApplication required Version/ApplicationFullName/ApplicationIdentifier (strings including explicit empty and multibyte text)');p.write_text(s,encoding='utf8')
