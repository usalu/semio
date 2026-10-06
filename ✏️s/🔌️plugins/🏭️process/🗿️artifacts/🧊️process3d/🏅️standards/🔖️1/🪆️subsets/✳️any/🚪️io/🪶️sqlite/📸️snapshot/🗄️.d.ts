/** 🗄️ Bun imports the authored SQL asset as UTF-8 text. */
declare module "*.sql" { const sql: string; export default sql; }
