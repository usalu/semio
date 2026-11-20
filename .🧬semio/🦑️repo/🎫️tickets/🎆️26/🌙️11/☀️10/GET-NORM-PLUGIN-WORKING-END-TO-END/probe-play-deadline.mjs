import {createRequire} from "node:module";
const require = createRequire(process.cwd()+"/package.json");
const {chromium} = require("playwright");
const browser = await chromium.launch({args:["--in-process-gpu","--enable-unsafe-swiftshader"]});
try {
  const page = await browser.newPage();
  const values = [];
  page.on("console", message => {
    const line=message.text();
    if(line.includes("[DEBUG] issued duration")) {values.push(line); console.log(line);}
    if(/boot failed|source-not-ready/.test(line)) console.log(line.split("\n")[0]);
  });
  await page.route(url => decodeURIComponent(url.pathname).endsWith("/🌉️bridge.js"), async route => {
    const upstream = await route.fetch();
    const js = await upstream.text();
    const pattern = /this\.deadlineUs = now \+ (?:3600000000n|admission\.maximumElapsedUs);/;
    if(!pattern.test(js)) throw Error("unrecognized deadline assignment");
    await route.fulfill({response:upstream,body:js.replace(pattern,'this.deadlineUs = now + admission.maximumElapsedUs; console.info("[DEBUG] issued duration", String(admission.maximumElapsedUs));')});
  });
  await page.goto("http://127.0.0.1:6033/#iso16757",{waitUntil:"commit"});
  await page.waitForFunction(()=>document.querySelector('[data-shell-ready],[data-shell-error]'),undefined,{timeout:120000});
  console.log(await page.locator('[data-shell-id="iso16757"]').evaluate(el=>({ready:el.dataset.shellReady,error:el.dataset.shellError})));
  if(!values.length) throw Error("No observed issued duration");
} finally {await browser.close();}
