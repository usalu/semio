const code=await Bun.file(process.argv[2]!).text(),needle=process.argv[3]!;let i=-1,n=0;
while((i=code.indexOf(needle,i+1))>=0&&n++<30)console.log(i,JSON.stringify(code.slice(Math.max(0,i-160),i+needle.length+80)),"\n");
