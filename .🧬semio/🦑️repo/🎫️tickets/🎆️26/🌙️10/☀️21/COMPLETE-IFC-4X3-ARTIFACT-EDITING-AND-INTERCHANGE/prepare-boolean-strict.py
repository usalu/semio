from pathlib import Path
import json
ticket=Path(__file__).parent
files=["control-authority.test.ts","cooperative.test.ts","🟦️.test.ts","manifold.test.ts","admission.test.ts","openings.test.ts"]
config={"extends":"../../../../../../../tsconfig.json","include":[],"exclude":[],"files":["../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests/"+name for name in files]}
(ticket/"boolean-tsconfig.json").write_text(json.dumps(config,ensure_ascii=False,indent=2)+"\n",encoding="utf8")

