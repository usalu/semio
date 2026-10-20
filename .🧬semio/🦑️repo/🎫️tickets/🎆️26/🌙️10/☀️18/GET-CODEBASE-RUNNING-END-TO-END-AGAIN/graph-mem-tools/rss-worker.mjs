// 📈️ Samples the process RSS from a worker thread so synchronous phases of the main thread are still observed; writes the peak into the shared buffer and ends the process above the bound.
import {workerData} from "node:worker_threads";
const {buffer,limit}=workerData,cells=new Float64Array(buffer);
setInterval(()=>{const rss=process.memoryUsage().rss;if(rss>cells[0])cells[0]=rss;if(rss>limit){process.stderr.write("ABORT rss "+Math.round(rss/1e6)+" MB\n");process.kill(process.pid,"SIGKILL");}},20);
