const fs=require('fs'),crypto=require('crypto'),p='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🦀️.rs',s=fs.readFileSync(p,'utf8');let n=s;
function one(a,b){if(n.split(a).length!==2)throw Error('unexpected source '+a);n=n.replace(a,b)}
one('sleb((136i64<<32)|2048','sleb((144i64<<32)|2048');one('sleb((144i64<<32)|2800','sleb((152i64<<32)|2800');one('sleb((136i64<<32)|2560','sleb((144i64<<32)|2560');
one('let mut data=vec![if input_receipt>=5{4}else{1},0,0x41]','let mut data=vec![if input_receipt>=5{5}else{2},0,0x41]');
one('sleb(2048,&mut data);data.push(0x0b);uleb(136,&mut data);','sleb(2048,&mut data);data.push(0x0b);uleb(144,&mut data);');
one('data.resize(data.len()+128,0);if input_receipt>=5{','data.resize(data.len()+128,0);data.extend(3200u32.to_le_bytes());data.extend(16u32.to_le_bytes());if input_receipt>=5{');
one('sleb(2560,&mut data);data.push(0x0b);uleb(136,&mut data);','sleb(2560,&mut data);data.push(0x0b);uleb(144,&mut data);');
one('data.extend([2,1,0,0]);data.resize(data.len()+128,0);data.extend([0,0x41]);sleb(2800','data.extend([2,1,0,0]);data.resize(data.len()+128,0);data.extend(3200u32.to_le_bytes());data.extend(16u32.to_le_bytes());data.extend([0,0x41]);sleb(2800');
one('sleb(2800,&mut data);data.push(0x0b);uleb(144,&mut data);','sleb(2800,&mut data);data.push(0x0b);uleb(152,&mut data);');
one('data.extend([5,1,0,0]);data.resize(data.len()+128,0);data.extend(2400u32.to_le_bytes());','data.extend([5,1,0,0]);data.resize(data.len()+128,0);data.extend(3200u32.to_le_bytes());data.extend(16u32.to_le_bytes());data.extend(2400u32.to_le_bytes());');
one('data.extend(4u32.to_le_bytes());}section(11,data,&mut core);','data.extend(4u32.to_le_bytes());}data.extend([0,0x41]);sleb(3200,&mut data);data.push(0x0b);uleb(16,&mut data);data.extend(b"SRE1");data.extend([0;4]);data.extend(1u64.to_le_bytes());section(11,data,&mut core);');
const t=p+'.edit-'+crypto.randomBytes(4).toString('hex');fs.writeFileSync(t,n,{flag:'wx'});if(fs.readFileSync(p,'utf8')!==s){fs.unlinkSync(t);throw Error('concurrent source edit')}fs.renameSync(t,p);
