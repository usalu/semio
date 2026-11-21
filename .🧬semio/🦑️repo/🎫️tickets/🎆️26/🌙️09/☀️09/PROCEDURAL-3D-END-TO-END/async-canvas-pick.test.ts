import { expect, test } from "bun:test";
import { EMPTY, Subject, switchMap } from "rxjs";
const cases = await Bun.file("🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📋️canvas-pick/🧪️tests/🧫️fixtures/async-pick.json").json();
for (const row of cases) {
  test(row.id, () => {
    const commands = new Subject<{ x: number; click: boolean } | null>();
    const pending = new Map<number, Subject<string>>();
    const received: string[] = [];
    let down: number | undefined;
    let click = false;
    const subscription = commands.pipe(switchMap(command => {
      click = command?.click ?? false;
      if (command === null) return EMPTY;
      const reply = new Subject<string>();
      pending.set(command.x, reply);
      return reply;
    })).subscribe(value => {
      if (click) received.push("hover:", "select:" + value);
      else received.push("hover:" + value);
      click = false;
    });
    for (const [kind, x, id] of row.steps) {
      if (kind === "move" && down === undefined && !click) commands.next({ x, click: false });
      if (kind === "down") { down = x; commands.next(null); }
      if (kind === "up") {
        if (down === undefined || Math.abs(x - down) <= 4) commands.next({ x, click: true });
        down = undefined;
      }
      if (kind === "leave" || kind === "cancel") { commands.next(null); received.push("hover:"); }
      if (kind === "cancel") down = undefined;
      if (kind === "unmount") subscription.unsubscribe();
      if (kind === "resolve") { pending.get(x)?.next(id); pending.get(x)?.complete(); }
      if (kind === "reject") pending.get(x)?.complete();
    }
    subscription.unsubscribe();
    expect(received).toEqual(row.events);
    expect([...pending.keys()]).toEqual(row.requests);
  });
}
