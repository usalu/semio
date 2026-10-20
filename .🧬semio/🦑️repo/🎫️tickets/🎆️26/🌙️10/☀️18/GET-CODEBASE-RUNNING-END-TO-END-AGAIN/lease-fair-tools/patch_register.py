import sys
root=sys.argv[1]
script=root+'/📜️script.ts'
project=root+'/📋️project.json'
s=open(script,encoding='utf8').read()
a='      "resource-lease-waits": {source:"🔒️leases/⏳️wait/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n'
assert s.count(a)==1 and '"resource-lease-fairness"' not in s
s=s.replace(a,a+'      "resource-lease-fairness": {source:"🔒️leases/⚖️fairness/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n')
open(script,'w',encoding='utf8').write(s)
p=open(project,encoding='utf8').read()
a='''        "command": "bun ./📜️script.ts test resource-lease-waits"
      },
      "metadata": {
        "semio": {
          "execution": {
            "version": 1,
            "kind": "source"
          }
        }
      }
    }
'''
assert p.count(a)==1 and 'test-resource-lease-fairness' not in p
new=a[:-len("    }\n")]+'''    },
    "test-resource-lease-fairness": {
      "executor": "repo:script",
      "cache": false,
      "dependsOn": [],
      "inputs": [
        "{projectRoot}/**/*"
      ],
      "outputs": [],
      "options": {
        "cwd": "🧰️framework/🔨️modules/🏃️process",
        "command": "bun ./📜️script.ts test resource-lease-fairness"
      },
      "metadata": {
        "semio": {
          "execution": {
            "version": 1,
            "kind": "source"
          }
        }
      }
    }
'''
p=p.replace(a,new)
open(project,'w',encoding='utf8').write(p)
import json;json.loads(p)
print("registered")
