/** 🗺️ Render the source-bound public physical atlas for independent inspection. */
import {render,renderStairs} from './physical-atlas-r92.mjs';
const root='C:/git/semio',ticket=new URL('.',import.meta.url).pathname.replace(/^\/([A-Z]:)/,'$1');
console.log('[DEBUG] '+await render(root,decodeURIComponent(ticket).replace(/\/$/,'')));
await renderStairs(root,decodeURIComponent(ticket).replace(/\/$/,''));
