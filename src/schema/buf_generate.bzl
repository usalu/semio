def buf_generate_impl(ctx):
    print("Generating code from protos with buf.")
    ctx.actions.run(
        executable = "buf",
        arguments = ["generate"], # "--include-imports"]
        inputs = [],
        outputs = ["dummy"]
    )
    return DefaultInfo()

buf_generate = rule(
    implementation = buf_generate_impl,
    
)