import{openSync}from"node:fs";
const count=Number(process.argv[2]);
for(let index=0;index<count;index++)openSync("/dev/null","r");
process.stdout.write(`READY ${process.pid}\n`);
setTimeout(()=>{},Number(process.argv[3]??20000));
