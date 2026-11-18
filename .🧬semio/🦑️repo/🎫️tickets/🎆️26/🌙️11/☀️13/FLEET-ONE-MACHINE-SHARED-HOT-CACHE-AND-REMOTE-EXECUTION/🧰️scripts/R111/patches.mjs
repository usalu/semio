const O="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
const edit=(from,to)=>text=>{const at=text.indexOf(from);if(at<0||text.indexOf(from,at+1)>=0)throw Error("anchor not unique: "+from.slice(0,60));return text.replace(from,()=>to);};
export const all=[
 {file:O,change:edit('else{await lookupLapAsyncV1("settle",()=>settleClosureRowsV1(header.observations,record.value));}for(const{kind,path,fact}of header.observations)','for(const{kind,path,fact}of header.observations)')},
];
export const apply=(file,text,list=all)=>list.filter(item=>file.endsWith(item.file)).reduce((value,item)=>item.change(value),text);
