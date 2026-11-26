const code=await Bun.file(process.argv[2]!).text();let i=-1,n=0;const re=/schema\/.{0,40}?json/g;
for(const m of code.matchAll(re)){const s=code.slice(Math.max(0,m.index-120),m.index+60);if(/file:"/.test(s.slice(-90))&&0)continue;if(!s.includes('from'))continue;console.log(m.index,JSON.stringify(s),"\n");if(++n>12)break;}
