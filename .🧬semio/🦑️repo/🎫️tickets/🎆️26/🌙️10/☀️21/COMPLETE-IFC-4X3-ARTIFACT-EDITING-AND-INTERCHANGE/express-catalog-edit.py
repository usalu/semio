from pathlib import Path
p=Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🟦️.ts")
s=p.read_text(encoding="utf-8")
head,tail=s.split("export type IfcDeclaration=",1)
import re
head=re.sub(r"(?m)^  (\w+\??:)",r"  readonly \1",head)
head=head.replace("IfcAttributeDeclaration[]","readonly IfcAttributeDeclaration[]").replace("members: string[]","members: readonly string[]").replace("inverse: string[]","inverse: readonly string[]").replace("whereRules: string[]","whereRules: readonly string[]").replace("uniqueRules: string[]","uniqueRules: readonly string[]")
head=head.replace("derived: { name: string; typeExpr: IfcTypeExpression; redeclares: string | null; expression: string }[]","derived: readonly Readonly<{name:string;typeExpr:IfcTypeExpression;redeclares:string|null;expression:string}>[]")
head=head.replace("source: { url: string; mirror: string; sha256: string };","source: Readonly<{url:string;mirror:string;sha256:string}>;")
head=head.replace("validation: { structural: boolean; whereRules: boolean; uniqueRules: boolean; inverseRules: boolean; contextualBounds: boolean; functions: boolean; globalRules: boolean; complexInstances: boolean; binaryValues: boolean };","validation: Readonly<{structural:boolean;whereRules:boolean;uniqueRules:boolean;inverseRules:boolean;contextualBounds:boolean;functions:boolean;globalRules:boolean;complexInstances:boolean;binaryValues:boolean}>;")
head=head.replace("entities: Record<string, IfcEntityDeclaration>;","entities: Readonly<Record<string,IfcEntityDeclaration>>;").replace("types: Record<string, IfcTypeDeclaration>;","types: Readonly<Record<string,IfcTypeDeclaration>>;").replace("functions: Record<string, string>;","functions: Readonly<Record<string,string>>;").replace("rules: Record<string, string>;","rules: Readonly<Record<string,string>>;")
assert head.count("readonly ")>=45,head.count("readonly ")
p.write_text(head+"export type IfcDeclaration="+tail,encoding="utf-8")
p=Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🛠️extract/🟦️.ts")
s=p.read_text(encoding="utf-8")
old="const entities:Record<string,IfcEntityDeclaration>={}"
assert old in s
s=s.replace(old,"const entities:Record<string,{-readonly[Key in keyof IfcEntityDeclaration]:IfcEntityDeclaration[Key]}>={}",1)
old="const flatten=(name:string):IfcAttributeDeclaration[]=>"
assert old in s
s=s.replace(old,"const flatten=(name:string):readonly IfcAttributeDeclaration[]=>",1)
p.write_text(s,encoding="utf-8")
p=Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🧪️tests/🟦️.test.ts")
s=p.read_text(encoding="utf-8")
old='broken.entities.IFCWALL.allAttributes[0].typeExpr={kind:"aggregate"};'
assert old in s
s=s.replace(old,'Reflect.set(broken.entities.IFCWALL.allAttributes[0],"typeExpr",{kind:"aggregate"});',1)
p.write_text(s,encoding="utf-8")
