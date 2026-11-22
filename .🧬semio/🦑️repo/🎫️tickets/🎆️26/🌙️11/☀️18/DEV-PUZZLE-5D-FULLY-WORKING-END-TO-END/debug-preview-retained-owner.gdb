set pagination off
set auto-load off
set debuginfod enabled off
set print elements 5
set print repeats 2
set env RUST_MIN_STACK 67108864
set breakpoint pending on
break '/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📦️packages/🦀️rust/../../🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs':496
commands
silent
python
try:
    turn = int(gdb.parse_and_eval("close_turn"))
except gdb.error:
    turn = None
if turn is None or turn >= 32768:
    gdb.execute("bt 5")
    gdb.execute("info args")
    for expr in ["app.tools.declared_tools", "app.tools.declared_owner", "app.history.command_log", "app.ingest.envelope_field_decoders", "app.tools.registry", "app.tools.tool_jobs", "app.child_composition.composition"]:
        try:
            v = gdb.parse_and_eval(expr)
            print("[DEBUG]", expr, str(v)[:1000])
        except gdb.error as error:
            print("[DEBUG]", expr, error)
end
continue
end
run
