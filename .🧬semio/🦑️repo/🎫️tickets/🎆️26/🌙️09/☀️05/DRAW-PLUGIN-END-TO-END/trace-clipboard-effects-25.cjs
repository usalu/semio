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
    context.on('page', p => p.on('console', m => {const t=m.text();if (/warn|error/.test(m.type()) || t.startsWith('[DEBUG]')) console.log(t.slice(0,1200));if(t.startsWith('[DEBUG] settled cut'))setTimeout(()=>{console.log('[DEBUG] fast-stop after cut');void browser.close()},5000)}));
    await context.route(u => decodeURIComponent(u.pathname).includes('clipboard-activation'), async route => {
      const response = await route.fetch();
      const src = await response.text();
      const body = src.replace('if (!isWorkspaceClipboardAction(action))', 'console.log("[DEBUG] queued", action.action, action.commandAddress, this.pending); if (!isWorkspaceClipboardAction(action))').replace('const tail = result.finally', 'void result.then(v => console.log("[DEBUG] settled", action.action, JSON.stringify(v))); const tail = result.finally');
      await route.fulfill({response,body});
    });
    await context.route(u => decodeURIComponent(u.pathname).includes('document-transfer-ref') || decodeURIComponent(u.pathname).includes('input-funnel') || decodeURIComponent(u.pathname).includes('operation-subscriptions'), async route => {
      const response = await route.fetch();
      const body = (await response.text()).replace('const action = pasteActionWithRetainedFragment', 'if (["copy","paste","cut"].includes(requested.action)) console.log("[DEBUG] run",requested.action,!!clipboardFragmentRef.current); const action = pasteActionWithRetainedFragment').replaceAll('settleOperation(completion);','console.log("[DEBUG] completion",String(completion.operation),JSON.stringify(completion.requestedEffects?.map(e => Object.keys(e))),completion.status); settleOperation(completion);');
      await route.fulfill({response,body});
    });
    await context.route(u => {
      const path = decodeURIComponent(u.pathname);
      return ['invocation-frames','handle-turn-drive','program-dispatch','host-effects','surface-refresh','module-loader'].some(part => path.includes('/'+part+'/') || path.split('/').some(name => name.endsWith(part))) && !path.includes('operation-subscriptions');
    }, async route => {
      const response = await route.fetch();
      let body = await response.text();
      const additions = [
        ['const requestedEffects = [];','if (typeof effects!=="undefined" && Array.isArray(effects) && effects.some(e=>e.tag==="clipboard-write")) console.log("[DEBUG] refresh clipboard",effects.map(e=>e.tag));'],
        ['const frames = routeHostEffects(instanceId, result.effects, binding.port)', 'if (result.effects.some(e=>e.tag==="clipboard-write")) console.log("[DEBUG] document clipboard");'],
        ['pendingTurnEffects.delete(instanceId);', 'if (["copy","cut","paste"].includes(actionId) || leftover.some(e=>e.tag==="clipboard-write")) console.log("[DEBUG] invoke effects",actionId,JSON.stringify(leftover.map(e=>e.tag)));'],
        ['const promoted = promoteShellSendMessages', 'if (leftover.some(e=>e.tag==="clipboard-write")) console.log("[DEBUG] command clipboard",JSON.stringify(inspected),outFrames.length);'],
        ['if (leftover.length > PLUGIN_OPERATION_EFFECT_CAPACITY)', 'if (leftover.some(e=>e.tag==="clipboard-write")) console.log("[DEBUG] drain clipboard",frames.length);'],
        ['const pending = frames.some', 'if (leftover.some(e=>e.tag==="clipboard-write")) console.log("[DEBUG] job clipboard",String(job),frames.length);'],
        ['const startedOperation = startedTypedOperationV1(response.output);', 'if (["copy","cut","paste"].includes(action.action) || response.requestedEffects?.some(e=>e.clipboardWrite)) console.log("[DEBUG] action effects",action.action,JSON.stringify(response.requestedEffects?.map(e=>Object.keys(e))),isCurrentEffectOwner(actionOwner));'],
        ['clipboardFragmentRef.current = clipboardWriteFragmentFromEffect(effect);', 'console.log("[DEBUG] apply clipboard",!!clipboardWriteFragmentFromEffect(effect),new Error().stack?.split("\\n").slice(1,7).join("|"));'],
      ];
      for(const [marker,statement] of additions) body=body.replace(marker,statement+' '+marker);
      await route.fulfill({response,body});
    });
    return context;
  };
  return browser;
};
import(journey).catch(err => {console.error(err);process.exitCode=1});
