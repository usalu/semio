const IFC="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc";
const {generateCatalog}=await import(IFC+"/📚️catalog/🛠️extract/📁️generate.ts");
for(const [dir,exp] of [["🔖️2x3/🪆️subsets/🧱️base","IFC2X3_TC1.exp"],["4️⃣4/🪆️subsets/✳️any","IFC4.exp"],["🔖️4x3/🪆️subsets/✳️any","IFC4X3_ADD2.exp"]]){const o=`${IFC}/🏅️standards/${dir}/🧬️schema/📚️catalog/`;console.log(dir,await generateCatalog({source:o+exp,manifest:o+"source.json",json:o+"🔣️.json",pack:o+"catalog.ifcpack"}));}
