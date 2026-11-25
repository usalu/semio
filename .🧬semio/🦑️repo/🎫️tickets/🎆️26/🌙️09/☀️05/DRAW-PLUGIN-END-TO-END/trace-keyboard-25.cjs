/** 🔬️ Observes the keyboard target and shell clipboard queue during the real acceptance journey. */
const {createRequire} = require('node:module');
const {resolve} = require('node:path');
const journey = resolve('✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs');
const {chromium} = createRequire(journey)('playwright');
const launch = chromium.launch.bind(chromium);
chromium.launch = async options => {
  const browser = await launch(options);
  const create = browser.newContext.bind(browser);
  browser.newContext = async opts => {
    const context = await create(opts);
    await context.addInitScript(() => {
      document.addEventListener('keydown', e => console.log('[DEBUG] key', e.key, e.target.id, e.target.getAttribute('data-ui-node-key')), true);
    });
    context.on('page', p => p.on('console', m => {if (/warn|error/.test(m.type()) || m.text().startsWith('[DEBUG]')) console.log(m.text().slice(0,600))}));
    await context.route(u => decodeURIComponent(u.pathname).includes('clipboard-activation'), async route => {
      const response = await route.fetch();
      const src = await response.text();
      const body = src.replace('if (!isWorkspaceClipboardAction(action))', 'console.log("[DEBUG] queued", action.action, action.commandAddress, this.pending); if (!isWorkspaceClipboardAction(action))').replace('const tail = result.finally', 'void result.then(v => console.log("[DEBUG] settled", action.action, JSON.stringify(v))); const tail = result.finally');
      await route.fulfill({response,body});
    });
    await context.route(u => decodeURIComponent(u.pathname).includes('document-transfer-ref') || decodeURIComponent(u.pathname).includes('input-funnel') || decodeURIComponent(u.pathname).includes('operation-subscriptions'), async route => {
      const response = await route.fetch();
      const body = (await response.text()).replace('const action = pasteActionWithRetainedFragment', 'if (["copy","paste","cut"].includes(requested.action)) console.log("[DEBUG] run",requested.action,!!clipboardFragmentRef.current); const action = pasteActionWithRetainedFragment').replaceAll('settleOperation(completion);','console.log("[DEBUG] completion",String(completion.operation),completion.requestedEffects?.map(e => Object.keys(e)),completion.status); settleOperation(completion);');
      await route.fulfill({response,body});
    });
    return context;
  };
  return browser;
};
import(journey).catch(err => {console.error(err);process.exitCode=1});
