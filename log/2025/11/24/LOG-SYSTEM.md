---
slug: LOG-SYSTEM
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: >-
  Implement comprehensive log management system with CRUD, nested structure, and
  YAML frontmatter
model: claude-sonnet-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Log System Implementation

**Date:** 2025-11-24
**Task:** Implement comprehensive log management system with CRUD operations, nested folder structure, and YAML frontmatter

## Overview

Expanded the log system from flat file structure to a nested date-based hierarchy with YAML frontmatter for metadata tracking.

## Changes Made

### 1. Created `scripts/log.ts`

Complete log management script with:

- **CRUD Operations**: create, read, update, delete, list
- **YAML Frontmatter**: Metadata tracking for date, slug, author, summary, model
- **Date-based Structure**: `log/YEAR/MONTH/DAY/SLUG.md`
- **Git Integration**: Auto-retrieves author from git config
- **Environment Variables**: `SEMIO_MODEL` for default LLM model
- **Migration Tool**: Converts old flat logs to new structure

#### Key Functions

```typescript
createLog(input: LogCreateInput): Log
readLog(year: number, month: number, day: number, slug: string): Log
updateLog(year: number, month: number, day: number, slug: string, update: LogUpdateInput): Log
deleteLog(year: number, month: number, day: number, slug: string): void
listLogs(options?: LogListOptions): Log[]
migrateOldLogs(): void
```

### 2. Frontmatter Format

Every log file now has YAML frontmatter:

```yaml
---
date: TIMESTAMP # ISO 8601 timestamp
slug: SLUG # Kebab-case identifier
author: NAME <EMAIL> # From git config
summary: SUMMARY # One-line description
model: MODEL # LLM model identifier
---
```

### 3. CLI Interface

```bash
npx tsx scripts/log.ts create SLUG "Summary"
npx tsx scripts/log.ts read YEAR MONTH DAY SLUG
npx tsx scripts/log.ts list [year] [month] [day]
npx tsx scripts/log.ts delete YEAR MONTH DAY SLUG
npx tsx scripts/log.ts migrate
```

### 4. Migration Complete

Migrated 31 existing logs from flat structure:

- Old: `log/2025-11-24_SLUG.md`
- New: `log/2025/11/24/SLUG.md`

All logs now have proper YAML frontmatter with metadata.

### 5. Documentation Updates

#### `AGENTS.md`

Added comprehensive "Log System" section covering:

- Directory structure
- Frontmatter format
- Script usage (CLI and programmatic)
- Environment variables
- Git configuration

Updated file structure documentation to show new hierarchy.

Updated general rules to reference new log creation command.

#### `.gitignore`

Changed from ignoring `log` directory to ignoring `*.log` files, allowing logs to be tracked in git.

### 6. Dependencies

Added `gray-matter` for YAML frontmatter parsing.

## Benefits

1. **Better Organization**: Date-based hierarchy makes logs easier to browse
2. **Rich Metadata**: Frontmatter enables filtering, searching, and attribution
3. **Version Control**: Logs now tracked in git for history and collaboration
4. **Automation Ready**: Programmatic API enables automated log management
5. **Model Tracking**: Records which LLM was used for each task
6. **Git Integration**: Auto-retrieves author information

## Usage Examples

### Create a log

```bash
npx tsx scripts/log.ts create MY-TASK "Implement new feature"
```

### Read a log

```bash
npx tsx scripts/log.ts read 2025 11 24 MY-TASK
```

### List logs from November 2025

```bash
npx tsx scripts/log.ts list 2025 11
```

### Programmatic usage

```typescript
import { createLog, listLogs } from "./scripts/log";

const log = createLog({
  slug: "MY-TASK",
  summary: "Implement new feature",
  content: "# Details\n\nImplementation notes...",
});

const logs = listLogs({ year: 2025, month: 11 });
```

## Files Changed

- Created: `scripts/log.ts`
- Modified: `AGENTS.md` (log system documentation, file structure, general rules)
- Modified: `.gitignore` (allow log directory, ignore \*.log files)
- Migrated: 31 log files to new structure
- Added: `package.json` dependency on `gray-matter`

## Future Enhancements

Potential improvements:

- Log templates for common task types
- Integration with commit messages
- Search functionality across logs
- Summary generation from logs
- Export to other formats (PDF, HTML)
