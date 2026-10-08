p='📜️script.ts'
s=open(p,encoding='utf8',newline='').read()
old_http='''    if (a === "http") {
      const rest = segments.slice(1);
      if ((rest[0] ?? "").trim().toLowerCase() === "os") {
        this.runMcpOs("http", rest.slice(1));
        return;
      }
      console.error("[dev] `dev mcp http` currently serves only the os gateway — use `dev mcp http os`.");
      process.exit(1);
    }
'''
assert s.count(old_http)==1
s=s.replace(old_http,'')
s=s.replace('''        this.runMcpOs("stdio", rest.slice(1));''','''        this.runMcpOs(rest.slice(1));''')
i=s.index('  /** 🌉️ Runs the `semio-os` MCP gateway')
j=s.index('  private runMcpStdioRepo')
new=r'''  /** 🌉️ Runs the `semio-os` MCP gateway (`semio-framework-os-mcp`) over stdio, which is what `.mcp.json` launches.
   * Streamable HTTP is the dashboard tool `tool:workspace/os-mcp-http` (Nx target `@semio-tech/framework-os-mcp-rs:dev`).
   * Extra argv passes straight through to the binary (`--folder`, `--hub`, `--principal`, `--scopes`, `--auto-approve`, …). */
  private runMcpOs(extra: string[]): void {
    runCmd(ensureMcpBinary(this.root), ["stdio", ...extra], { cwd: this.root, ...daemonBudgetOpts() });
  }

'''
s=s[:i]+new+s[j:]
open(p,'w',encoding='utf8',newline='').write(s)
