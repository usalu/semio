const n=Number(process.env.BALLAST_OBJECTS??0);globalThis.__ballast=Array.from({length:n},(_,i)=>({a:i,b:"row-"+i,c:[i,i+1]}));
console.log("ballast objects",n,"rss MB",Math.round(process.memoryUsage().rss/1e6));
