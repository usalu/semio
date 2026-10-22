import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "mut"))
from recipes import *

calls = [call("interactivityDatabaseCapabilityOpenFailures", "dbEngine"), call("interactivityDatabaseCatalogReadFailures", "dbEngine")]
generate("⚖laws/🔬interactivity-audit/🗄database/🔓capability-catalog", "🔓capability-catalog", "🔓 The capability-open and catalog-read laws accept the live engine family and reject it when a required anchor disappears or a forbidden one appears.", db_sources(), calls)
