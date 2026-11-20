import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./🔣️.json";

test("whole-dictionary connections export each source channel once", () => {
    const scope = fixture.exportScope;
    const database = new Database(":memory:");
    try {
        database.exec("create table wire (ordinal integer, source text, port text, target text); create table output (source text, channel text, handle text);");
        const wire = database.query("insert into wire values (?, ?, ?, ?)");
        for (const [ordinal, row] of scope.host.synapses.entries()) wire.run(ordinal, row.from, row.fromPort, row.to);
        const output = database.query("insert into output values (?, ?, ?)");
        for (const [source, channels] of Object.entries(scope.outputs)) {
            for (const [channel, value] of Object.entries(channels)) if (typeof value === "string") output.run(source, channel, value);
        }
        const connected = database.query(`
          with selected as (select * from wire where target = ?),
          sources as (select source, min(ordinal) as first from selected group by source)
          select distinct output.handle from output join sources using (source)
          where exists (select 1 from selected where selected.source=output.source and (port = '' or port = channel))
          order by sources.first, output.channel
        `);
        const preview = database.query("select handle from output where source = ? order by channel");
        for (const row of scope.cases) {
            const actual = "widgetId" in row.request && row.request.widgetId
                ? connected.all(row.request.widgetId)
                : scope.host.widgets.filter(widget => "preview" in widget && widget.preview).flatMap(widget => preview.all(widget.id));
            expect(actual.map(value => (value as { handle: string }).handle)).toEqual(row.handles);
        }
    } finally {
        database.close();
    }
});
