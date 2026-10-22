"""📍️ Round 3: the schema bases are pinned at the positions that depend on the manager (regions), not as whole documents, so neutral edits by any other set around them no longer break the contract: materialization digests, the pre-seam original values and the Python oracle all speak regions."""
def declare(edit, paths):
    K = paths.MANAGERS
    M = K + "/🧩️materialize/🟦️.ts"
    edit(M, "round3-regions", "seam3", " return walk(base);\n}\n", ''' return walk(base);
}

/** 📍️ Lists the positions of a schema base that depend on the manager, each with the value it materializes to: the regression surface of a base, independent of every neutral edit around it (a list item that is itself such a position is addressed by its own text, any other by index). */
export function managerRegionsV1(base:unknown,view:SchemaViewV1):readonly (readonly [string,unknown])[]{
 const rows:(readonly [string,unknown])[]=[];
 const isRegion=(node:unknown):boolean=>isExpressionV1(node)||(typeof node==="string"&&new RegExp(tokenV1.source,"u").test(node));
 const visit=(node:unknown,path:string):void=>{
  if(isRegion(node)){rows.push([path,materializeSchemaV1(node,view)]);return;}
  if(Array.isArray(node)){node.forEach((item,index)=>visit(item,path+"/["+(isExpressionV1(item)?"$"+item.$manager:isRegion(item)?String(item):String(index))+"]"));return;}
  if(node!==null&&typeof node==="object")for(const [key,value] of Object.entries(node))visit(value,path+"/"+key);
 };
 visit(base,"");
 return rows;
}
''')
    O = K + "/🧪️tests/🐍️oracle.py"
    edit(O, "round3-oracle-regions", "seam3", "def schema_bases():\n", '''def is_region(node):
    return ("$manager" in node) if isinstance(node, dict) else isinstance(node, str) and TOKEN.search(node) is not None


def regions(node, view, path=""):
    if is_region(node):
        return [[path, materialize(node, view)]]
    rows = []
    if isinstance(node, list):
        for index, item in enumerate(node):
            step = "$" + item["$manager"] if isinstance(item, dict) and "$manager" in item else item if is_region(item) else str(index)
            rows += regions(item, view, path + "/[" + step + "]")
    elif isinstance(node, dict):
        for key, value in node.items():
            rows += regions(value, view, path + "/" + key)
    return rows


def schema_bases():
''')
    edit(O, "round3-oracle-digest", "seam3", 'sha(canonical(materialize(base, view))) for key, base in schema_bases().items()', 'sha(canonical(regions(base, view))) for key, base in schema_bases().items()')
    T = K + "/🧪️tests/🟦️.ts"
    edit(T, "round3-test-import", "seam3", "materializeSchemaV1,schemaViewV1}", "managerRegionsV1,materializeSchemaV1,schemaViewV1}")
