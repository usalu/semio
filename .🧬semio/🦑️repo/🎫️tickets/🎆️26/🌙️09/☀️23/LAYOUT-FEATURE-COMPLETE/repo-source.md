# Repo Source Analysis Startup

The repo build front door stopped while importing the build observation module. The module compiled the pattern for `portableSourceAnalysisOwner` at startup, and the observation schema lacked that definition. The portable project-record exporter already writes the parser role followed by the `⟦analysis-owner⟧` token.

The observation schema now declares the role and exact portable owner token. The parser identity fixture tests a portable record against a local parser of the same role, and its existing SQLite oracle independently confirms the expected verdict. Named lookalikes and exact providers replaced at another revision still fail the same law.

A direct repo script served as the diagnostic file reader while the normal tool registry was unavailable. After the schema repair the normal Python tool query and all five parser identity tests passed. The running Layout builds retained their leases.
# Process Package Export

A later repo startup failed because the public Process artifact-storage module did not expose the existing `nativeImageFreshSpaceV1` implementation. The public re-export now exposes that exact function. The ticket’s import identity check passed, and `repo tools exec --light python` started successfully.

