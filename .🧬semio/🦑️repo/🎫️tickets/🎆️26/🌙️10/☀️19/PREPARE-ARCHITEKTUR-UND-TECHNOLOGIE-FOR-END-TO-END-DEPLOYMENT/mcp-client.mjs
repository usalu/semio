import { spawn } from 'node:child_process';

/** 🔌 Connects the ticket coordinator to the native repository interface. */
export async function connectRepo(executable, root) {
  const child = spawn(executable, ['mcp'], { cwd: root, windowsHide: true, stdio: ['pipe', 'pipe', 'pipe'] });
  const pending = new Map();
  let sequence = 0, buffer = '', stderr = '';
  const fail = error => { for (const item of pending.values()) { clearTimeout(item.timer); item.reject(error); } pending.clear(); };
  child.once('error', fail);
  child.once('exit', (code, signal) => fail(new Error('Repository interface exited: ' + code + '/' + signal + ' ' + stderr)));
  child.stdin.on('error', fail);
  child.stderr.setEncoding('utf8');
  child.stderr.on('data', chunk => { stderr = (stderr + chunk).slice(-8192); });
  child.stdout.setEncoding('utf8');
  child.stdout.on('data', chunk => {
    buffer += chunk;
    let end;
    while ((end = buffer.indexOf('\n')) >= 0) {
      const line = buffer.slice(0, end).trim(); buffer = buffer.slice(end + 1);
      if (!line) continue;
      let message;
      try { message = JSON.parse(line); } catch { fail(new Error('Invalid repository protocol output')); continue; }
      const item = pending.get(message.id);
      if (!item) continue;
      clearTimeout(item.timer); pending.delete(message.id);
      if (message.error) item.reject(new Error(JSON.stringify(message.error))); else item.resolve(message.result);
    }
  });
  const notify = (method, params) => child.stdin.write(JSON.stringify({ jsonrpc: '2.0', method, params }) + '\n');
  const call = (method, params) => new Promise((resolve, reject) => {
    const id = ++sequence;
    const timer = setTimeout(() => { pending.delete(id); notify('notifications/cancelled', { requestId: id, reason: 'Coordinator diagnostic request deadline' }); reject(new Error('Repository request deadline exceeded')); }, 45000);
    pending.set(id, { resolve, reject, timer });
    child.stdin.write(JSON.stringify({ jsonrpc: '2.0', id, method, params }) + '\n');
  });
  const initialization = await call('initialize', { protocolVersion: '2024-11-05', capabilities: {}, clientInfo: { name: 'codex-main-coordination', version: '1.0' } });
  notify('notifications/initialized', {});
  return { child, pending, call, notify, initialization, close: () => child.stdin.end() };
}
