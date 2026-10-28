"""obsrows-g: derives the staged observation reuse test from the live one by exact, unique replacements (re-runnable)."""
import os, sys
OB = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
CA = os.environ["CA"]
rel = "👁️observation/🧪️tests/📥️reuse/🟦️.ts"
text = open(OB + rel, encoding="utf8").read()
def sub(old, new, count=1):
    global text
    if text.count(old) != count:
        sys.exit("anchor count %d != %d: %s" % (text.count(old), count, old[:90]))
    text = text.replace(old, new)

# 1. the reuse predicate of a graph caller is a conjunction that contains the inventory hold; the one farm-worker caller judges the record shape only because its parent (the caller with `prepare`) re-judges every adopted record with the hold
sub('return calls.length===2&&calls.every(call=>{const options=call.arguments[0];if(!options||!ts.isObjectLiteralExpression(options))return false;return options.properties.some(property=>ts.isPropertyAssignment(property)&&property.name.getText(file)==="reusable"&&ts.isArrowFunction(property.initializer)&&ts.isCallExpression(property.initializer.body)&&property.initializer.body.expression.getText(file)==="inferenceInventoryReferencesHoldV1"&&property.initializer.body.arguments[1]?.getText(file)==="value");});};',
    'const holds=(node:ts.Expression):boolean=>ts.isParenthesizedExpression(node)?holds(node.expression):ts.isCallExpression(node)?node.expression.getText(file)==="inferenceInventoryReferencesHoldV1"&&node.arguments[1]?.getText(file)==="value":ts.isBinaryExpression(node)&&node.operatorToken.kind===ts.SyntaxKind.AmpersandAmpersandToken&&(holds(node.left)||holds(node.right));const reusable=(call:ts.CallExpression)=>{const options=call.arguments[0];if(!options||!ts.isObjectLiteralExpression(options))return undefined;const property=options.properties.find(item=>ts.isPropertyAssignment(item)&&item.name.getText(file)==="reusable");return property&&ts.isPropertyAssignment(property)?{initializer:property.initializer,prepared:options.properties.some(item=>ts.isPropertyAssignment(item)&&item.name.getText(file)==="prepare")}:undefined;};const holding=(call:ts.CallExpression)=>{const row=reusable(call);return !!row&&ts.isArrowFunction(row.initializer)&&row.initializer.parameters.length===1&&row.initializer.parameters[0]!.name.getText(file)==="value"&&holds(row.initializer.body);};const shape=(call:ts.CallExpression)=>{const row=reusable(call);return !!row&&ts.isIdentifier(row.initializer)&&row.initializer.text==="projectRecordReusableV1"&&!row.prepared;};return calls.length===3&&calls.every(call=>holding(call)||shape(call))&&calls.filter(shape).length===1&&calls.some(call=>reusable(call)?.prepared)&&calls.filter(call=>reusable(call)?.prepared).every(holding);};')
sub('expect(proof(source.replaceAll("reusable:value=>inferenceInventoryReferencesHoldV1","reusable:value=>unrelatedInventoryPredicateV1"))).toBe(false);',
    'expect(proof(source.replaceAll("reusable:value=>inferenceInventoryReferencesHoldV1","reusable:value=>unrelatedInventoryPredicateV1"))).toBe(false);expect(proof(source.replaceAll("&&inferenceInventoryReferencesHoldV1","&&unrelatedInventoryPredicateV1"))).toBe(false);expect(proof(source.replaceAll("&&inferenceInventoryReferencesHoldV1","||inferenceInventoryReferencesHoldV1"))).toBe(false);expect(proof(source.replaceAll("reusable: projectRecordReusableV1","ignored: projectRecordReusableV1"))).toBe(false);')

# 2. the portable expansion keeps a bounded memo of expanded closures: extracted with the function, asserted for hits and for its bound
sub('const code=ts.transpileModule(node.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS}}).outputText,readPolicy=',
    'const memoStatements=tree.statements.filter(row=>ts.isVariableStatement(row)&&row.declarationList.declarations.some(item=>["expandedClosuresV1","expandedClosureEntriesV1"].includes(item.name.getText(tree)))||ts.isFunctionDeclaration(row)&&row.name?.text==="rememberExpandedClosureV1");if(memoStatements.length!==3)throw Error("Actual expanded closure memo required");const code=ts.transpileModule([...memoStatements,node].map(row=>row.getText(tree)).join("\\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS}}).outputText,readPolicy=')
sub('checkResourceLeaseControl:(control:unknown)=>{expect(control).toBe(actual);',
    'GRAPH_MEMORY_POLICY_V1:memoryPolicy,graphStringInternerV1,checkResourceLeaseControl:(control:unknown)=>{expect(control).toBe(actual);')
sub('data=createSourceInventoryDataV1(readPolicy),valid=',
    'data=createSourceInventoryDataV1(readPolicy),{GRAPH_MEMORY_POLICY_V1:memoryPolicy,graphStringInternerV1}=await import("../../../🧠️memory/🟨️.mjs"),valid=')
sub('if(row.count)expect(published[0]).toBe(`build:graph-phase:expansion:0:${row.count}`);}',
    'if(row.count)expect(published[0]).toBe(`build:graph-phase:expansion:0:${row.count}`);'
    'const firstReads=reads;expect((await run(root,{inputs},{},{})).inputs).toEqual(expected);expect(reads,"a memoized closure is not read again").toBe(firstReads);expect(remembered).toBe(row.count);'
    'const limit=Math.max(1,Math.ceil(row.count/2)),model=(rounds:number)=>{const memo=new Set<number>();let held=0,count=0;for(let round=0;round<rounds;round++)for(let index=0;index<row.count;index++){if(memo.has(index))continue;count++;if(held+1>limit){memo.clear();held=0;}memo.add(index);held++;}return count;};'
    'let boundedReads=0;const bounded=new Function(...Object.keys(ports),code+";return portableInputs;")(...Object.values({...ports,GRAPH_MEMORY_POLICY_V1:{...memoryPolicy,maximumCachedClosureEntries:limit},readInventoryRecordLeanV1:(directory:string,key:string,cap:number)=>{boundedReads++;return ports.readInventoryRecordLeanV1(directory,key,cap);}}));'
    'for(let round=0;round<2;round++)expect((await bounded(root,{inputs},{},{})).inputs).toEqual(expected);expect(boundedReads,"the memo clears when it would exceed its entry bound").toBe(model(2));}')

d = CA + "/obsrows-g-tools/staged/🏗️build/" + os.path.dirname(rel); os.makedirs(d, exist_ok=True)
open(CA + "/obsrows-g-tools/staged/🏗️build/" + rel, "w", encoding="utf8").write(text)
print("ok", rel)
