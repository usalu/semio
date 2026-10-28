/** 🔮️ A separate typed-memory allocation and lodash byte census validate the portable native close witnesses. */
import lodash from 'lodash';
const{sumBy}=lodash;
export function physicalBufferOracle(fixture){return fixture.cases.map(row=>{const bytes=row.capacity*(row.kind==='offset'?Uint32Array.BYTES_PER_ELEMENT:Uint8Array.BYTES_PER_ELEMENT),allocation=Buffer.allocUnsafeSlow(bytes),views=row.kind==='offset'?[new Uint32Array(allocation.buffer,allocation.byteOffset,row.capacity)]:[new Uint8Array(allocation.buffer,allocation.byteOffset,row.capacity)];return{name:row.name,release:sumBy(views,view=>view.byteLength)};});}
