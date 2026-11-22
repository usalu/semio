set pagination off
set auto-load off
set debuginfod enabled off
set env RUST_MIN_STACK 67108864
set breakpoint pending on
break rust_panic
run
bt 18
python
frame = gdb.newest_frame()
while frame is not None and "close_registered_fixture_app" not in str(frame.name()):
    frame = frame.older()
if frame is not None:
    frame.select()
    gdb.execute("info args")
    for expr in ["app.tools.declared_tools", "app.tools.declared_owner", "app.history.command_log", "app.ingest.envelope_field_decoders", "app.tools.registry", "app.tools.tool_jobs", "app.child_composition.composition"]:
        try:
            v = gdb.parse_and_eval(expr)
            print(expr, str(v)[:500])
        except gdb.error as error:
            print(expr, error)
end
