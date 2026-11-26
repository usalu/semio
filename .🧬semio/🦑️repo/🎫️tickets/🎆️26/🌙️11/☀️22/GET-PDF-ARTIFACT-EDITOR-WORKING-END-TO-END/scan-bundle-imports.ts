const code=await Bun.file(process.argv[2]!).text(),scan=new Bun.Transpiler({loader:"js"}).scan(code);
console.log(JSON.stringify(scan.imports.filter(row=>!row.path.startsWith("node:")),null,1));
