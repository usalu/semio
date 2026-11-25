const o={};
o.boot=await until(async()=>/Case: dead/.test(await txt()),180000,3000);
return o;
