def _task_impl(ctx):
    output = ctx.actions.declare_directory(ctx.label.name + ".snapshot")
    dependencies = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    args = [ctx.file.runner.path, ctx.file.source.path, ctx.attr.source_identity, "birth-action", ctx.file.manifest.path, output.path] + [dep.path for dep in dependencies]
    requirements = {"local": "1", "no-sandbox": "1", "no-remote": "1"}
    if not ctx.attr.cache:
        requirements["no-cache"] = "1"
    ctx.actions.run(executable = ctx.attr.bun, arguments = args, inputs = depset([ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source] + dependencies), outputs = [output], mnemonic = "RepoTask", progress_message = ctx.attr.identity, execution_requirements = requirements, use_default_shell_env = True)
    return [DefaultInfo(files = depset([output]))]

repo_task = rule(implementation = _task_impl, attrs = {"manifest": attr.label(allow_single_file = True), "fingerprint": attr.label(allow_single_file = True), "deps": attr.label_list(), "runner": attr.label(allow_single_file = True), "source": attr.label(allow_single_file = True), "source_identity": attr.string(mandatory = True), "bun": attr.string(mandatory = True), "identity": attr.string(mandatory = True), "cache": attr.bool(default = False)})

def _launch_impl(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ctx.attr.extension)
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.runner.path, ctx.file.source.path, ctx.attr.source_identity, "birth-launcher", ctx.file.manifest.path, launcher.path], inputs = [ctx.file.manifest, ctx.file.runner, ctx.file.source], outputs = [launcher], mnemonic = "RepoLauncher", execution_requirements = {"local": "1", "no-sandbox": "1", "no-remote": "1"})
    dependencies = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    return [DefaultInfo(executable = launcher, runfiles = ctx.runfiles(files = dependencies + [ctx.file.manifest, ctx.file.runner, ctx.file.source]))]

repo_launch = rule(implementation = _launch_impl, executable = True, attrs = {"manifest": attr.label(allow_single_file = True), "deps": attr.label_list(), "bun": attr.string(mandatory = True), "runner": attr.label(allow_single_file = True), "source": attr.label(allow_single_file = True), "source_identity": attr.string(mandatory = True), "extension": attr.string(default = "")})
