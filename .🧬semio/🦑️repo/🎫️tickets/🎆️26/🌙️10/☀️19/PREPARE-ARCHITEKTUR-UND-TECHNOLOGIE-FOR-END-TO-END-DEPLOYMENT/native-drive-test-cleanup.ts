import { dlopen, FFIType as f } from 'bun:ffi';
const api=dlopen('kernel32.dll',{QueryDosDeviceW:{args:[f.ptr,f.ptr,f.u32],returns:f.u32},DefineDosDeviceW:{args:[f.u32,f.ptr,f.ptr],returns:f.i32}});
const wide=(s:string)=>Buffer.from(s+'\0','utf16le'), name=wide('Z:'), buffer=Buffer.alloc(65536), target='\\??\\D:\\semio';
const size=api.symbols.QueryDosDeviceW(name,buffer,buffer.length/2), actual=buffer.toString('utf16le').split('\0',1)[0];
if (!size || actual!==target) throw Error('Mapping differs from the test-owned target: '+actual);
if (!api.symbols.DefineDosDeviceW(7,name,wide(target))) throw Error('Exact cleanup failed');
api.close(); console.log('Test-owned Z: mapping removed');
