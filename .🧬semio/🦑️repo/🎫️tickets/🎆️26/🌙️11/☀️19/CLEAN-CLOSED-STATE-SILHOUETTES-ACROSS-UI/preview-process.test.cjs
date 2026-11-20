const assert = require("node:assert/strict");
const fs = require("node:fs");
const {EventEmitter} = require("node:events");
const {test} = require("node:test");
const {apps,createPreviewLifecycle} = require("./preview-process.cjs");
const cases = require("./preview-cases.json");

test("all app previews reference a real config and working folder",()=>{
  for (const app of apps) {
    assert.ok(fs.statSync(app.cwd).isDirectory(),app.id);
    assert.ok(fs.statSync(app.config).isFile(),app.id);
  }
});

for (const scenario of cases) test(scenario.name,async ()=>{
  const events = [];
  const lifecycle = createPreviewLifecycle({
    inspect: async app=>({ready:scenario.external?.includes(app.id)}),
    spawnPreview: app=>{
      events.push("start " + app.id);
      const child = new EventEmitter();
      child.kill = signal=>{
        assert.equal(signal,"SIGTERM");
        events.push("stop " + app.id);
        queueMicrotask(()=>child.emit("exit",0));
      };
      return child;
    },
  });
  const commands = {
    batch: app=>lifecycle.startBatch(app),
    manual: app=>lifecycle.startManual(app,false),
    restart: app=>lifecycle.startManual(app,true),
    stopManual: app=>lifecycle.stopManual(app.id),
    stopBatch: ()=>lifecycle.stopBatch(),
    shutdown: ()=>lifecycle.shutdown(),
  };
  for(const [name,id] of scenario.steps) await commands[name](apps.find(app=>app.id===id));
  assert.deepEqual(events,scenario.events);
});
