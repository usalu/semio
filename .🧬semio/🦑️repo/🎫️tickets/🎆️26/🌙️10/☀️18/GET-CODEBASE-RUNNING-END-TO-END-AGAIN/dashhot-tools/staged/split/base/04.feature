Feature: Native Dashboard Installation and Launch

  Scenario: The selected binary owns its dependency lock input
    Given root-owned and nested-workspace-owned binary fixtures
    When their exact Cargo source inputs are captured
    Then each binary retains its selected workspace lock and dependency workspace manifests
    And unrelated workspace locks and manifests do not invalidate that binary
  Scenario: Bootstrap authority closes authored conditional build dependencies before Original birth
    Given run, daemon, preferences and install declare build as an owned child
    And build and installed test routes declare compile as an owned child
    When the caller declares its initial dashboard selection
    Then exact build and compile requests are closed before issuing its Original
    And child projection preserves backend, mode and native resource ceilings
    And no native argv or extra sibling request gains authority
    And unit, windows, compile, unrelated and nonexistent workflow targets acquire no conditional child
  Scenario Outline: Missing modes install before read-only or stop controls
    Given the selected mode has no dashboard installation
    When the developer requests <control>
    Then the native owner installs that mode once before acquiring the executable reader
    And the rebuild retains the original invocation and cancellation signal
    And failure or cancellation prevents the native child from starting

    Examples:
      | control       |
      | tasks         |
      | logs          |
      | stop          |
      | kill          |
      | open          |
      | daemon status |
      | daemon stop   |

  Scenario: Existing stale controls remain available
    Given the selected mode has an installed dashboard behind the current sources
    When the developer observes or stops an existing task or daemon
    Then no source staleness check or rebuild blocks the selected control
    And its native child retains installed-reader custody and original cancellation

  Scenario: Ordinary launch of an installed dashboard
    Given an installation record references an immutable native executable
    When a developer starts the dashboard or its daemon controls
    Then no project graph, dependency provisioning, build or executable hash runs
    And the digest of the sources it was built from is recomputed without reading any file
    And the native arguments match the shared launch vectors
  Scenario: Publishing a new executable while a view is alive
    Given the first installed executable is running
    When a new build publishes and installs another executable
    Then the running executable retains its original bytes
    And new launches select the new installation record
    And Nx remains responsible for build and installation tasks
  Scenario: Starting after a source of the dashboard changed
    Given an installation record names the digest of the dashboard sources and of the sources of its path dependencies
    When a file below one of those source trees is edited, added or only touched
    Then the next start reports why the installation is stale and rebuilds and reinstalls with visible progress
    And a file in tests, fixtures, build output or the dev dependencies of the crate does not make it stale
    And a failed or cancelled rebuild stops the start instead of running the outdated executable

  Scenario: Start an installed dashboard after source changes
    Given a dashboard installation exists
    And a compiled source changed since installation
    When the developer runs the dashboard bootstrap
    Then the bootstrap creates or receives an original process envelope
    And its rebuild receives that same policy, deadline and capabilities
    And the rebuilt executable starts only after successful installation

  Scenario: An inherited deadline remains authoritative
    Given the dashboard bootstrap received a process envelope
    When it takes the installed executable route
    Then it retains the original deadline without extending it
    And refuses conflicting explicit policy or capabilities

  Scenario: Cancel a bootstrap child
    Given the dashboard owns a rebuild or terminal subprocess
    When its original process invocation is cancelled
    Then its owned subprocess tree is stopped
    And the bootstrap reports exit code 130
    And a normal child retains its actual exit code

  Scenario: Cancel a dashboard view without stopping its workspace
    Given a dashboard view started an independently supervised workspace daemon
    When the view's original invocation is cancelled
    Then the view process ends
    And the workspace daemon and its tasks remain controllable

  Scenario: The full native suite declares its actual minimum tier
    Given the native suite includes real terminal and multi-view integration journeys
    When a developer runs the default native suite
    Then its minimum test level is quick
    And an explicitly requested higher level is retained

  Scenario: Install the current optimized release while obsolete output exists
    Given the canonical native builder publishes its release profile output
    And an obsolete unprofiled executable remains in a different directory
    When the dashboard installation owner publishes immutable executable bytes
    Then the installed bytes equal the current release bytes
    And their address equals the independent SHA-256 digest
    And the installation directory agrees with the independently compiled builder

  Scenario: Sources changed during preparation are captured at compilation
    Given the owner prepares a canonical source revision before the compiler starts
    When preparation and profile queueing finish
    Then the full source identity names the revision consumed by the compiler
    And source edits during compilation refuse installation
    And cancellation remains bound to the original compiler invocation

  Scenario: Validate current metadata without serial filesystem waits
    Given a source workspace has many real project owner directories
    When the bootstrap revalidates their current custody
    Then at least two independent observations can overlap
    And at most sixteen observations are outstanding
    And every observation is settled before publication
    And symlink replacement, peer edits and cancellation still refuse publication

  Scenario: Dependency workspace metadata changes invalidate the installed dashboard
    Given a path dependency inherits package metadata and table-form dependencies from another workspace
    When its Cargo workspace manifest or inherited local source changes
    Then the installed dashboard source digest changes
    And unrelated sources and the dependency workspace's independent lock do not change the digest
    And startup checks the recorded exact manifests without reading or scanning the whole dependency workspace

  Scenario: Observation and stop controls recover invalid installation metadata
    Given an installation record is missing, unreadable or invalid for this host and mode
    When a developer selects a stale-capable dashboard control
    Then the existing owner rebuilds before admitting the immutable executable
    And valid Source-stale installations retain their observation and stop access
    And the metadata probe reads no executable bytes outside tooling custody

  Scenario: Atomic edits retain source freshness despite preserved timestamps
    Given a source file or exact dependency workspace manifest is replaced with different equal-size bytes
    And its modification timestamp is preserved
    When the installed Source signature is checked
    Then device, inode and nanosecond metadata invalidate the old digest
    And replacing an independent dependency lock remains outside the captured scope

  Scenario: Files the executable includes from outside its crate roots make the installation stale
    Given an installation records the files of its executable's own Cargo dep-info and when that build started
    When a schema outside every crate root or a TypeScript file that the binary includes is edited, replaced with its old timestamps or removed
    Then the installation is stale although the source digest is unchanged, and the reason names that file
    And reading an included file or editing an unrelated file keeps the installation current

  Scenario: A build of a current installation runs no nested build
    Given the installation of the selected mode records exactly the captured sources and none of its compiled files changed
    And its immutable executable is in place with the recorded size and permissions
    When the dashboard build task runs
    Then it answers that executable without starting the nested compile and says so in one line in the language of the command
    And a removed, resized or re-permissioned executable, a changed source or a changed compiled file sends it on to build
