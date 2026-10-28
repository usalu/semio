from pathlib import Path
import hashlib,json
import sys
ticket=Path(__file__).parent
root=ticket.parents[6]
artifact=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
registry=artifact/"📦️packages/🟦️typescript/📜️script.ts"
before=registry.read_bytes()
anchor=b'  "'+"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/validation.test.ts".encode()+b'",'
addition=b'  "'+"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️coverage/🧪️.test.ts".encode()+b'",'
if '--inputs' not in sys.argv and addition not in before:
 assert before.count(anchor)==1
 newline=b"\r\n" if b"\r\n" in before else b"\n"
 after=before.replace(anchor,anchor+newline+addition)
 assert registry.read_bytes()==before
 registry.write_bytes(after)
if '--inputs' not in sys.argv:assert registry.read_bytes().count(addition)==1
target=artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests"
files=["csg-round-cases.json","csg-round.schema.json","csg-round.feature","csg-round-supplemental-cases.json","csg-round-supplemental.schema.json"]
out=[]
for name in files:
 source=ticket/name
 assert source.is_file(),name
 destination=target/name
 assert len(str(destination))<=256
 data=source.read_bytes()
 if destination.exists():assert destination.read_bytes()==data,name
 else:destination.write_bytes(data)
 out.append({"name":name,"sha256":hashlib.sha256(data).hexdigest()})
print(json.dumps({"registration":"deferred" if '--inputs' in sys.argv else "issued-owner suite exactly once","copied":out}))
