/** 🔎️ Maps observed Chromium WebAssembly indices to local global-data references and function bodies. */
const [wasmPath, profilePath] = Bun.argv.slice(2);
const bytes = await Bun.file(wasmPath).bytes();
const profile = await Bun.file(profilePath).json();
const selected = new Set(profile.nodes.flatMap(({callFrame}) => {
  const match = /wasm-function\[(\d+)\]/.exec(callFrame.functionName);
  return match ? [Number(match[1])] : [];
}));
for (const index of [118901,118902,119181,119183,119206,119376,267355]) selected.add(index);
if (Bun.argv[4]) selected.add(Number(Bun.argv[4]));
const decode = new TextDecoder();
const bodies = new Map(), data = [], exports = new Map();
let cursor = 8, imports = 0;
function leb(signed = false) {
  let value = 0;
  for (let shift = 0; shift <= 35; shift += 7) {
    const byte = bytes[cursor++];
    if (byte === undefined) throw Error("Unexpected WebAssembly extent");
    value += (byte & 127) * 2 ** shift;
    if (byte < 128) return signed && (byte & 64) !== 0 ? value - 2 ** (shift + 7) : value;
  }
  throw Error("Unsupported WebAssembly integer");
}
function name() {
  const length = leb(), start = cursor;
  cursor += length;
  return decode.decode(bytes.subarray(start, cursor));
}
function limits() {
  const flags = leb();
  leb();
  if (flags & 1) leb();
}
while (cursor < bytes.length) {
  const section = bytes[cursor++], size = leb(), end = cursor + size;
  if (section === 2) {
    for (let count = leb(), i = 0; i < count; i++) {
      name();
      name();
      const kind = bytes[cursor++];
      switch (kind) {
        case 0: leb(); imports += 1; break;
        case 1: cursor += 1; limits(); break;
        case 2: limits(); break;
        case 3: cursor += 2; break;
        case 4: cursor += 1; leb(); break;
        default: throw Error("Unknown WebAssembly import kind " + kind);
      }
    }
  }
  if (section === 7) {
    for (let count = leb(), i = 0; i < count; i++) {
      const symbol = name(), kind = bytes[cursor++], index = leb();
      if (kind === 0) exports.set(index, symbol);
    }
  }
  if (section === 10) {
    for (let count = leb(), i = 0; i < count; i++) {
      const length = leb(), index = imports + i;
      if (selected.has(index)) bodies.set(index, { start: cursor, end: cursor + length });
      cursor += length;
    }
  }
  if (section === 11) {
    for (let count = leb(), i = 0; i < count; i++) {
      const flags = leb();
      if (flags === 2) leb();
      let base;
      if (flags !== 1) {
        if (bytes[cursor++] !== 65) throw Error("Expected a static data address");
        base = leb(true);
        if (bytes[cursor++] !== 11) throw Error("Expected a single offset expression");
      }
      const length = leb();
      if (base !== undefined) data.push({ offset: base, start: cursor, length });
      cursor += length;
    }
  }
  cursor = end;
}
const parents = new Map(profile.nodes.flatMap(node => (node.children ?? []).map(child => [child, node.id])));
const samples = new Map(profile.nodes.map(node => [node.id, Number(/wasm-function\[(\d+)\]/.exec(node.callFrame.functionName)?.[1])]));
const counts = new Map();
for (let sample of profile.samples.slice(-2000)) {
  while (sample !== undefined) {
    const idx = samples.get(sample);
    if (Number.isInteger(idx)) counts.set(idx, (counts.get(idx) ?? 0) + 1);
    sample = parents.get(sample);
  }
}
for (const [index, score] of [...counts].sort((a, b) => b[1] - a[1]).slice(0, 95)) {
  const body = bodies.get(index);
  if (!body) continue;
  const strings = new Set();
  for (let at = body.start; at < body.end; at++) {
    if (bytes[at] !== 65) continue;
    cursor = at + 1;
    let address;
    try { address = leb(true); } catch { continue; }
    const segment = data.find(row => address >= row.offset && address < row.offset + row.length);
    if (!segment) continue;
    const mapped = segment.start + address - segment.offset;
    let end = mapped;
    while (end < mapped + 125 && bytes[end] >= 32 && bytes[end] <= 126) end++;
    if (end < mapped + 14) continue;
    const value = decode.decode(bytes.subarray(mapped, end));
    if (/close|stall|acknowledg|retire|grant|rung|publication|allocation|capacity|owner|maintenance/i.test(value)) strings.add(value);
  }
  if (strings.size || score > 100) console.log(index, score, body.end - body.start, exports.get(index) ?? "", [...strings].slice(0, 16));
}
const focus = [118900,118901,118902,119181,119183,119206,119376,267355,119355,119976,127879,128006,127993,267394,267368,267515,267375,267316];
for (const id of focus) {
  const body=bodies.get(id);
  if (!body) continue;
  const numbers=[];
  for(let at=body.start;at<body.end;at++){
    if(bytes[at]!==65&&bytes[at]!==66&&bytes[at]!==16)continue;
    cursor=at+1;
    try{
      const n=leb(bytes[at]!==16);
      if(bytes[at]===16&&((n>118700&&n<120100)||(n>127700&&n<128100)||(n>267300&&n<267600))) numbers.push([at-body.start,"call",n]);
      else if(bytes[at]!==16&&n>=8192&&n<2000000) numbers.push([at-body.start,"const",n]);
    }catch{}
  }
  console.log("constants",id,body.start,body.end-body.start,JSON.stringify(numbers));
}
function dataOffset(memory) {
  const segment=data.find(row=>memory>=row.offset&&memory<row.offset+row.length);
  return segment && segment.start + memory - segment.offset;
}
const view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);
for(const id of focus){
  const body=bodies.get(id);
  if(!body)continue;
  const places=new Set();
  for(let at=body.start;at<body.end;at++){
    if(bytes[at]!==65)continue;
    cursor=at+1;
    let address;
    try{address=leb(true);}catch{continue;}
    const offset=dataOffset(address);
    if(offset===undefined||offset+16>bytes.length)continue;
    const pointer=view.getUint32(offset,true), length=view.getUint32(offset+4,true), mapped=dataOffset(pointer);
    if(mapped===undefined||length<5||length>512)continue;
    const file=decode.decode(bytes.subarray(mapped,mapped+length));
    if(file.endsWith(".rs"))places.add(file+":"+view.getUint32(offset+8,true));
  }
  console.log("places",id,[...places]);
}
const operators=new Map([[0,"unreachable"],[1,"nop"],[2,"block"],[3,"loop"],[4,"if"],[5,"else"],[11,"end"],[12,"br"],[13,"br_if"],[14,"br_table"],[15,"return"],[16,"call"],[17,"call_indirect"],[26,"drop"],[27,"select"],[32,"local.get"],[33,"local.set"],[34,"local.tee"],[35,"global.get"],[36,"global.set"],[40,"i32.load"],[41,"i64.load"],[45,"i32.load8_u"],[54,"i32.store"],[55,"i64.store"],[58,"i32.store8"],[65,"i32.const"],[66,"i64.const"],[69,"i32.eqz"],[70,"i32.eq"],[71,"i32.ne"],[73,"i32.lt_u"],[75,"i32.gt_u"],[77,"i32.le_u"],[79,"i32.ge_u"],[80,"i64.eqz"],[81,"i64.eq"],[82,"i64.ne"],[106,"i32.add"],[107,"i32.sub"],[108,"i32.mul"],[113,"i32.and"],[114,"i32.or"],[115,"i32.xor"],[116,"i32.shl"],[118,"i32.shr_u"],[124,"i64.add"],[125,"i64.sub"],[167,"i32.wrap_i64"]]);
function instructions(id,first,last){
 const body=bodies.get(id);
 if(!body)throw Error("Unknown function");
 cursor=body.start;
 for(let n=leb(),i=0;i<n;i++){leb();cursor++;}
 while(cursor<body.end&&cursor-body.start<=last){
  const pc=cursor,op=bytes[cursor++],args=[];
  if([2,3,4,65].includes(op)) args.push(leb(true));
  else if([12,13,16,32,33,34,35,36,37,38,63,64,210].includes(op)) args.push(leb());
  else if(op===14){const size=leb();for(let n=0;n<=size;n++)args.push(leb());}
  else if(op===17||(op>=40&&op<=62))args.push(leb(),leb());
  else if(op===66){let value=0n;for(let shift=0n;;shift+=7n){const byte=bytes[cursor++];value|=BigInt(byte&127)<<shift;if(byte<128){if(byte&64)value-=1n<<(shift+7n);break;}}args.push(value.toString());}
  else if(op===67)cursor+=4;
  else if(op===68)cursor+=8;
  else if(op===252){const sub=leb();args.push(sub);if([8,10,12,14].includes(sub))args.push(leb(),leb());else if(sub>=9&&sub<=17)args.push(leb());}
  else if(op===253||op===254)throw Error("Instruction needs a SIMD or atomic decoder at "+pc);
  if(pc-body.start>=first)console.log("instruction",pc,pc-body.start,operators.get(op)??op.toString(16),args.join(" "));
 }
}
if(Bun.argv[4])instructions(Number(Bun.argv[4]),Number(Bun.argv[5]??0),Number(Bun.argv[6]??500));
