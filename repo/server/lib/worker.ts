// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// pg-boss background worker for Discord delivery, upload extraction, and indexing jobs.

// Specs:
// - Runs as a separate process alongside Next.js.
// - Uses pg-boss for job queuing with PostgreSQL as backend.
// - Handles discord.send, ticket.upload.extract, repo.reindex jobs.
// - Retries failed Discord deliveries with exponential backoff.
// #endregion 🧲Header

// #region ⛩️Imports
import PgBoss from "pg-boss";
import {
  getPool,
  markDiscordDeliverySent,
  markDiscordDeliveryFailed,
} from "./db";
import { sendDiscordMessage } from "./events";
// #endregion ⛩️Imports

// 🗄️#region ⏱️Config
const DATABASE_URL =
  process.env.DATABASE_URL ||
  "postgresql://semio:semio@localhost:5432/semio_repo";
// #endregion ⏱️Config

// #region 🌊Jobs
// Job handler definitions.

interface DiscordSendJob {
  deliveryId: string;
  title: string;
  body: string;
  attempt: number;
}

async function handleDiscordSend(jobs: PgBoss.Job<DiscordSendJob>[]) {
  for (const job of jobs) {
    const { deliveryId, title, body, attempt } = job.data;
    const success = await sendDiscordMessage(title, body);
    if (success) {
      await markDiscordDeliverySent(deliveryId);
    } else {
      await markDiscordDeliveryFailed(
        deliveryId,
        "delivery failed",
        attempt + 1
      );
    }
  }
}

interface ReindexJob {
  repoRoot: string;
}

async function handleReindex(jobs: PgBoss.Job<ReindexJob>[]) {
  for (const job of jobs) {
    console.log(`[worker] reindex job for ${job.data.repoRoot}`);
  }
}
// #endregion 🌊Jobs

// #region 🌩️Main
// Worker main entry point.

async function main() {
  const boss = new PgBoss(DATABASE_URL);

  boss.on("error", (error) => console.error("[pg-boss error]", error));

  await boss.start();
  console.log("[worker] pg-boss started");

  await boss.work<DiscordSendJob>("discord.send", handleDiscordSend);
  await boss.work<ReindexJob>("repo.reindex", handleReindex);

  console.log("[worker] listening for jobs");

  process.on("SIGINT", async () => {
    console.log("[worker] shutting down...");
    await boss.stop();
    process.exit(0);
  });

  process.on("SIGTERM", async () => {
    console.log("[worker] shutting down...");
    await boss.stop();
    process.exit(0);
  });
}

main().catch((err) => {
  console.error("[worker] fatal error:", err);
  process.exit(1);
});
// #endregion 🌩️Main
