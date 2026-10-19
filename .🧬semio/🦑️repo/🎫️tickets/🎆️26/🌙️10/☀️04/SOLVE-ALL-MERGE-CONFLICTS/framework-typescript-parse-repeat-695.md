# Framework TypeScript Source Parser Audit

Used the existing TypeScript library only to parse source text. No compiler emission, typechecker, project build, tests, application runtime, or dependency resolver.

{
  "parsed": 363,
  "issues": [
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts",
      "diagnostics": [
        {
          "position": 753,
          "line": 18,
          "message": "Invalid character."
        },
        {
          "position": 754,
          "line": 18,
          "message": "Property assignment expected."
        },
        {
          "position": 769,
          "line": 18,
          "message": "Expression expected."
        },
        {
          "position": 782,
          "line": 18,
          "message": "',' expected."
        },
        {
          "position": 820,
          "line": 18,
          "message": "',' expected."
        },
        {
          "position": 829,
          "line": 19,
          "message": "Property assignment expected."
        },
        {
          "position": 909,
          "line": 20,
          "message": "Argument expression expected."
        },
        {
          "position": 910,
          "line": 20,
          "message": "Argument expression expected."
        },
        {
          "position": 914,
          "line": 21,
          "message": "Argument expression expected."
        },
        {
          "position": 915,
          "line": 21,
          "message": "Argument expression expected."
        },
        {
          "position": 923,
          "line": 22,
          "message": "',' expected."
        },
        {
          "position": 4842,
          "line": 80,
          "message": "Argument expression expected."
        }
      ]
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx",
      "diagnostics": [
        {
          "position": 1295,
          "line": 17,
          "message": "',' expected."
        }
      ]
    }
  ]
}

After preserving B/H/S source intent for the two merge-introduced defects: {"parsed":363,"issues":[]}

After current-owner and token-subspan preservation, excluding delegated Plugin/Renderer: {"parsed":274,"issues":[{"path":"🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts","diagnostics":[{"line":26,"message":"Argument expression expected."},{"line":26,"message":"':' expected."},{"line":26,"message":"',' expected."},{"line":26,"message":"Expression expected."}]},{"path":"🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts","diagnostics":[{"line":10,"message":"';' expected."},{"line":10,"message":"';' expected."},{"line":10,"message":"Declaration or statement expected."},{"line":10,"message":"Argument expression expected."},{"line":12,"message":"Declaration or statement expected."},{"line":12,"message":"Declaration or statement expected."},{"line":17,"message":"Expression expected."},{"line":17,"message":"')' expected."}]},{"path":"🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🧪️tests/🏗️initialization/🟦️.ts","diagnostics":[{"line":7,"message":"',' expected."},{"line":7,"message":"Declaration or statement expected."},{"line":7,"message":"';' expected."},{"line":7,"message":"Declaration or statement expected."},{"line":7,"message":"Declaration or statement expected."},{"line":7,"message":"'try' expected."},{"line":7,"message":"Declaration or statement expected."},{"line":7,"message":"Declaration or statement expected."}]},{"path":"🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🧪️tests/🟦️.ts","diagnostics":[{"line":5,"message":"';' expected."},{"line":6,"message":"')' expected."},{"line":16,"message":"Unterminated template literal."}]},{"path":"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts","diagnostics":[{"line":83,"message":"')' expected."},{"line":149,"message":"',' expected."}]},{"path":"🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎬️introduction-demo/🟦️.tsx","diagnostics":[{"line":139,"message":"',' expected."},{"line":139,"message":"',' expected."},{"line":139,"message":"',' expected."},{"line":139,"message":"',' expected."},{"line":369,"message":"Declaration or statement expected."},{"line":369,"message":"Declaration or statement expected."},{"line":369,"message":"';' expected."},{"line":428,"message":"Declaration or statement expected."}]},{"path":"🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎨️chrome/🟦️.tsx","diagnostics":[{"line":14,"message":"',' expected."},{"line":14,"message":"',' expected."},{"line":14,"message":"',' expected."},{"line":14,"message":"',' expected."},{"line":14,"message":"',' expected."},{"line":14,"message":"Property assignment expected."},{"line":14,"message":"',' expected."},{"line":15,"message":"'{' expected."},{"line":15,"message":"':' expected."},{"line":15,"message":"',' expected."},{"line":16,"message":"':' expected."},{"line":16,"message":"',' expected."},{"line":17,"message":"':' expected."},{"line":17,"message":"',' expected."},{"line":19,"message":"',' expected."},{"line":19,"message":"':' expected."},{"line":19,"message":"',' expected."},{"line":20,"message":"':' expected."},{"line":20,"message":"',' expected."},{"line":21,"message":"':' expected."},{"line":21,"message":"',' expected."},{"line":22,"message":"':' expected."},{"line":22,"message":"',' expected."},{"line":23,"message":"':' expected."},{"line":23,"message":"',' expected."},{"line":23,"message":"',' expected."},{"line":44,"message":"',' expected."},{"line":44,"message":"',' expected."},{"line":44,"message":"',' expected."},{"line":44,"message":"',' expected."},{"line":50,"message":"';' expected."},{"line":50,"message":"',' expected."},{"line":50,"message":"',' expected."},{"line":50,"message":"Module declaration names may only use ' or \" quoted strings."},{"line":53,"message":"Unexpected keyword or identifier."},{"line":53,"message":"Module declaration names may only use ' or \" quoted strings."},{"line":54,"message":"Unexpected keyword or identifier."},{"line":54,"message":"Module declaration names may only use ' or \" quoted strings."},{"line":70,"message":"',' expected."},{"line":70,"message":"Expression expected."},{"line":70,"message":"Variable declaration expected."},{"line":70,"message":"';' expected."},{"line":100,"message":"';' expected."},{"line":100,"message":"Declaration or statement expected."},{"line":100,"message":"',' expected."},{"line":100,"message":"',' expected."},{"line":100,"message":"',' expected."},{"line":100,"message":"',' expected."},{"line":100,"message":"',' expected."},{"line":130,"message":"Unterminated template literal."}]}]}

After rejecting all fine-token fallback, current core source parse: {"parsed":272,"issues":[]}
