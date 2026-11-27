const { toolJobPuzzleReservedRoutesSelfTests } = await import("@semio-tech/puzzle-js/testing/tool-job-puzzle-reserved-routes");
const { toolJobPuzzleReservedRoutesExact, toolJobPuzzle2dReservedRoutesExact } = await import("@semio-tech/repo-lib/laws");
console.log(`puzzle-reserved-route-contributions: checks=${toolJobPuzzleReservedRoutesSelfTests({ toolJobPuzzleReservedRoutesExact, toolJobPuzzle2dReservedRoutesExact })}`);
