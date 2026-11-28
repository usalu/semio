import {systemDarwinFootprintPortV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🍏️native-footprint/🟦️.ts";
const port=systemDarwinFootprintPortV1();
console.log("port",port===null?"null":"ok");
if(port){for(const pid of [process.pid,63402,11702,75886,79215,1,176,40314,99999999])console.log(pid,port.footprint(pid));}
