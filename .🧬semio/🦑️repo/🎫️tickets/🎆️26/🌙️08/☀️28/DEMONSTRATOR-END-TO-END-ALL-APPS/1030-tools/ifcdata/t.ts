const IFC="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc";
const {extractExpressCatalog}=await import(IFC+"/📚️catalog/🛠️extract/🟦️.ts");
const t=`SCHEMA X;\nTYPE A = STRING; END_TYPE;\nFUNCTION F(a:INTEGER):BOOLEAN;\n (* c *)\n -- l\n RETURN(TRUE);\nEND_FUNCTION;\nENTITY E;\n x:STRING;\nDERIVE\n y:STRING := 'a'\n -- k\n ;\nEND_ENTITY;\nEND_SCHEMA;`;
console.log(JSON.stringify(extractExpressCatalog(t,{schema:"X",version:"1",source:{url:"",mirror:"",sha256:""}}).functions));
