const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const { createMachine, createActor, assign } = require('xstate');
const root = path.resolve(__dirname, '../../../../../../..');
const fixture = JSON.parse(fs.readFileSync(path.join(root, '✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️paint/🧪️tests/🧫️fixtures/terminal.json'), 'utf8'));
for (const item of fixture.cases) {
  const chart = createMachine({
    context: { utility: fixture.initialUtility, mutations: 0, samples: 0 },
    initial: 'idle',
    states: {
      idle: { on: { PRESS: 'painting' } },
      painting: { on: {
        SWITCH: { actions: assign({ utility: () => fixture.terminalUtility }) },
        ABORT: 'idle',
        COMMIT: { target: 'idle', actions: assign({ mutations: () => 1, samples: ({context}) => Number(context.utility === 'eyedropper') }) },
      } },
    },
  });
  const actor = createActor(chart).start();
  actor.send({ type: 'PRESS' });
  actor.send({ type: 'SWITCH' });
  actor.send({ type: item.phase.toUpperCase() });
  const output = actor.getSnapshot();
  assert.equal(output.value, 'idle');
  assert.equal(output.context.mutations, item.mutations);
  assert.equal(output.context.samples, item.samples);
  actor.stop();
}
console.log('Independent XState terminal contract: ' + fixture.cases.length + ' cases passed.');
