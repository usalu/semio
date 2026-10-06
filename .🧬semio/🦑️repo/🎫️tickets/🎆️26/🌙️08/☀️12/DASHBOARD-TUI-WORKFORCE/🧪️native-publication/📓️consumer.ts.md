
if(process.argv[2]==="consumer-proof")edit(`${owner}/🧪️tests/🧊️native-runtime/🟦️.ts`,text=>{
 const anchor='    assert.equal(createHash("sha256").update(bytes).digest("hex"), descriptor.hashes.wasmSha256);\n  }';assert.ok(text.includes(anchor));return text.replace(anchor,anchor.replace('\n  }','\n    console.log("[DEBUG] Native consumer fetched its live asset");\n  }'));
});
