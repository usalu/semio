# SPDX-License-Identifier: AGPL-3.0-only
# #region 🔖Neo4jBootstrap
"""🗄️Idempotently creates semio/elements/coda/reuse databases on the devcontainer Neo4j service (system DB)."""
from __future__ import annotations

import os
import sys
import time

from neo4j import GraphDatabase


def main() -> None:
    uri = os.environ.get("NEO4J_URI", "bolt://neo4j:7687")
    user = os.environ.get("NEO4J_USERNAME", "neo4j")
    password = os.environ.get("NEO4J_PASSWORD", "password")
    driver = None
    for _ in range(90):
        try:
            driver = GraphDatabase.driver(uri, auth=(user, password))
            driver.verify_connectivity()
            break
        except Exception:
            time.sleep(2)
            if driver is not None:
                driver.close()
                driver = None
    if driver is None:
        print("Neo4j bootstrap: could not reach " + uri, file=sys.stderr)
        raise SystemExit(1)
    try:
        with driver.session(database="system") as session:
            for name in ("semio", "elements", "coda", "reuse"):
                session.run(f"CREATE DATABASE `{name}` IF NOT EXISTS")
    finally:
        driver.close()
    print("Neo4j bootstrap: databases semio, elements, coda, reuse are ready.")


if __name__ == "__main__":
    main()

# #endregion 🔖Neo4jBootstrap
