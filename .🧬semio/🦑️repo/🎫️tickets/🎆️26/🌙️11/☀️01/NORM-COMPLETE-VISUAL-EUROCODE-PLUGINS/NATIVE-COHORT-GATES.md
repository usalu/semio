# Native Cohort Gates

The shared five-package native suite runs first. The fifteen artifact workspaces are separate from the norm workspace. Registered native project targets own each artifact manifest and package roster. Source presence and compile progress do not establish a passing build or test. All ordinary source and host budgets remain enabled.

| Cohort | Build | Check | Test |
| --- | --- | --- | --- |
| en1990 | Pending | Pending | Pending |
| en1991 | Pending | Pending | Pending |
| en1992 | Pending | Pending | Pending |
| en1993 | Pending | Pending | Pending |
| en1994 | Pending | Pending | Pending |
| en1995 | Pending | Pending | Pending |
| en1996 | Pending | Pending | Pending |
| en1997 | Pending | Pending | Pending |
| en1998 | Pending | Pending | Pending |
| en1999 | Pending | Pending | Pending |
| din4108 | Pending | Pending | Pending |
| din16798 | Pending | Pending | Pending |
| din18599 | Pending | Pending | Pending |
| iso16757 | Pending | Pending | Pending |
| vdi3805 | Pending | Pending | Pending |

Direct EN1990 engineering/inference test: pending after shared native suite.

The additional cohort map is documented in [the five-cohort audit](🔍️audits/e2e-five-cohort-native-audit.md).

Shared native suite: ninth run in progress (handle 37717); eighth run compiled all five test binaries and executed 36 successful entity tests plus one stale assertion now corrected. Parent owns a new descendant edit-rule feature; the next shared verification will include live-core tests once those fixtures are ready.

## Registered Batch Scheduling

The repository CLI documents run-many with comma-separated targets/projects and a parallel task bound. Its help was read through `repo run-many --help`; the generated capture is temporary. One registered invocation will schedule the complete build/check/test matrix after the main rerun planner releases host memory:

```sh
repo run-many --targets=build,check,test --projects=@semio-tech/norm-en1990-rs,@semio-tech/norm-en1991-rs,@semio-tech/norm-en1992-rs,@semio-tech/norm-en1993-rs,@semio-tech/norm-en1994-rs,@semio-tech/norm-en1995-rs,@semio-tech/norm-en1996-rs,@semio-tech/norm-en1997-rs,@semio-tech/norm-en1998-rs,@semio-tech/norm-en1999-rs,@semio-tech/norm-din4108-rs,@semio-tech/norm-din16798-rs,@semio-tech/norm-din18599-rs,@semio-tech/norm-iso16757-rs,@semio-tech/norm-vdi3805-rs --configuration=prototype --parallel=1 --output=stream
```

This preserves registered target dependencies, package rosters and all custody/admission/capacity gates. No direct script entry or threshold override is used. The batch has not started. Shared native run 2228 currently processes the six production packages, including the extracted outliner.
