
if(process.argv[2]==="runtime")await (await import(join(root,owner,"🧪️tests/🧊️native-runtime/🟦️.ts"))).testNativeRuntime(root,generated);
if(process.argv[2]==="runtime-contract")edit(`${owner}/🧪️tests/🧊️native-runtime/🟦️.ts`,text=>{const before='assert.equal(target.continuous, operation === "run");';assert.ok(text.includes(before));return text.replace(before,'assert.equal(target.continuous, false);');});
