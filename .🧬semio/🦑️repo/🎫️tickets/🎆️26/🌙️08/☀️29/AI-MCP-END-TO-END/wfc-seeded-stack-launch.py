import os
import resource
import sys

soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
budget = min(hard, 32 * 1024 ** 2) if hard != resource.RLIM_INFINITY else 32 * 1024 ** 2
print(f"[DEBUG] Native gateway inherited stack limits {soft}/{hard}", file=sys.stderr, flush=True)
resource.setrlimit(resource.RLIMIT_STACK, (budget, hard))
print(f"[DEBUG] Native gateway exec stack limit {soft} -> {budget}", file=sys.stderr, flush=True)
os.execv(sys.argv[1], sys.argv[1:])
