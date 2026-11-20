import {createRequire} from "node:module";
const {chromium}=createRequire(process.cwd()+"/package.json")("playwright");
const browser=await chromium.launch({args:["--in-process-gpu","--enable-unsafe-swiftshader"]});
try {
  const page=await browser.newPage({locale:"en-US"});
  await page.goto("http://127.0.0.1:6033/#iso16757",{waitUntil:"commit"});
  await page.locator('[data-shell-ready]').waitFor({timeout:90000});
  await page.getByRole("button",{name:"Artifact",exact:true}).click();
  console.log(await page.locator("body").innerText().then(t=>t.slice(0,5000)));
  console.log(await page.getByRole('button').evaluateAll(items=>items.map(el=>[el.id,el.title,el.ariaLabel]).filter(parts=>parts.some(p=>p&&/open|import|transfer/i.test(p))).slice(0,32)));
} finally {await browser.close();}
