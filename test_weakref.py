import weakref
import asyncio
from mcp.server.fastmcp import FastMCP, Context

mcp = FastMCP("test")

@mcp.tool()
def test_weakref(ctx: Context):
    try:
        r = weakref.ref(ctx.session)
        return {"weakref_possible": True}
    except TypeError:
        return {"weakref_possible": False}

async def run_test():
    # We can't easily run the full MCP server and connect to it here,
    # but we can try to see what ctx.session is.
    # Actually, let's just check if we can find the class of ctx.session.
    pass

if __name__ == "__main__":
    print("This script is just a placeholder for logic that needs to be checked in the actual environment.")
