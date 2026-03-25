const { chromium } = require('playwright');
const fs = require('fs');

(async () => {
  let html = fs.readFileSync('/workspaces/semio/semio/engine/dist/mcp-app.html', 'utf8');
  
  const configJson = JSON.stringify({
    toolId: 'test-tool-id',
    toolName: 'show_design',
    toolInput: {},
    toolOutput: null,
    theme: 'dark',
    viewMode: 'inline',
    viewParams: {}
  }).replace(/</g, '\\u003C').replace(/>/g, '\\u003E');

  const compatScript = `(function() {
    if (window.openai) return;
    var el = document.getElementById('openai-compat-config');
    if (!el) { console.warn('[COMPAT] no config'); return; }
    var config;
    try { config = JSON.parse(el.textContent); } catch(e) { console.error('[COMPAT] parse', e); return; }
    var callId = 0;
    var pendingCalls = new Map();
    
    window.addEventListener('message', function(event) {
      if (event.source !== window.parent) return;
      var data = event.data;
      if (!data || data.jsonrpc !== '2.0') return;
      if (data.id != null && (data.result !== undefined || data.error !== undefined)) {
        var pending = pendingCalls.get(data.id);
        if (pending) {
          pendingCalls.delete(data.id);
          if (data.error) pending.reject(new Error(String(data.error)));
          else pending.resolve(data.result);
        }
        return;
      }
      if (data.method === 'ui/notifications/tool-result') {
        console.error('[DEBUG] COMPAT got tool-result');
      }
    });
    
    var initId = ++callId;
    console.error('[DEBUG] COMPAT sending ui/initialize id=' + initId);
    window.parent.postMessage({
      jsonrpc: '2.0', id: initId,
      method: 'ui/initialize',
      params: { appInfo: { name: 'openai-compat', version: '1.0.0' }, appCapabilities: {}, protocolVersion: '2026-01-26' }
    }, '*');
    pendingCalls.set(initId, {
      resolve: function(result) {
        console.error('[DEBUG] COMPAT init resolved');
        window.parent.postMessage({ jsonrpc: '2.0', method: 'ui/notifications/initialized', params: {} }, '*');
      },
      reject: function() {
        console.error('[DEBUG] COMPAT init rejected');
      }
    });
    
    Object.defineProperty(window, 'openai', { value: {}, writable: false, configurable: false, enumerable: true });
    console.error('[DEBUG] COMPAT loaded, window.openai set');
  })();`;
  
  const headInjection = `<script type="application/json" id="openai-compat-config">${configJson}<\/script><script>${compatScript}<\/script>`;
  html = html.replace(/<head>/i, '<head>' + headInjection);

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  
  const logs = [];
  page.on('console', msg => logs.push(msg.type() + ': ' + msg.text()));
  page.on('pageerror', err => logs.push('PAGE_ERROR: ' + err.message));
  
  // Write host HTML to file
  const toolResult = JSON.stringify({
    points: [{guid:'p1',id:'piece-1',u:0,v:0,status:'default'}],
    lines: [],
    capabilities: {pieceSelection:true},
    kitArtifacts: {designs:[{guid:'d1',name:'Test'}],types:[],ports:[]}
  });
  
  const hostHtml = `<!DOCTYPE html><html><body>
<iframe id="app" style="width:800px;height:600px;border:none;"></iframe>
<script>
var iframe = document.getElementById('app');
var initCount = 0;

window.addEventListener('message', function(e) {
  if (e.source !== iframe.contentWindow) return;
  var data = e.data;
  if (!data || data.jsonrpc !== '2.0') return;
  
  console.log('[HOST] Recv: ' + JSON.stringify(data).substring(0, 200));
  
  if (data.method === 'ui/initialize') {
    initCount++;
    console.log('[HOST] ui/initialize #' + initCount + ' id=' + data.id + ' from=' + JSON.stringify(data.params.appInfo));
    iframe.contentWindow.postMessage({
      jsonrpc: '2.0', id: data.id,
      result: { protocolVersion: '2026-01-26', hostInfo: { name: 'test', version: '1.0.0' }, hostCapabilities: {}, hostContext: { theme: 'light' } }
    }, '*');
  }
  
  if (data.method === 'ui/notifications/initialized') {
    console.log('[HOST] initialized notification #' + initCount);
    if (initCount >= 2) {
      setTimeout(function() {
        console.log('[HOST] Sending tool-result');
        iframe.contentWindow.postMessage({
          jsonrpc: '2.0',
          method: 'ui/notifications/tool-result',
          params: {
            content: [{ type: 'text', text: '${toolResult.replace(/'/g, "\\'")}' }]
          }
        }, '*');
      }, 500);
    }
  }
});

iframe.srcdoc = ${JSON.stringify(html)};
<\/script></body></html>`;

  fs.writeFileSync('/tmp/host-compat-test.html', hostHtml);
  
  await page.goto('file:///tmp/host-compat-test.html', { timeout: 15000 });
  await page.waitForTimeout(8000);
  
  // Check iframe
  const frames = page.frames();
  if (frames.length > 1) {
    const bodyText = await frames[1].evaluate(() => document.body?.innerText?.substring(0, 500));
    console.log('\n=== IFRAME TEXT ===');
    console.log(bodyText || '(empty)');
  } else {
    console.log('\n=== NO IFRAME FRAME FOUND ===');
  }
  
  console.log('\n=== LOGS ===');
  for (const l of logs) console.log(l);
  
  await browser.close();
})();
