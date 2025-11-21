// SQL query loader - reads SQL files from the sql/sqlite folder
import { readFileSync } from 'fs';
import { join } from 'path';

const SQL_DIR = join(__dirname, '../../../sql/sqlite');

export const loadSQL = (filename: string): string => {
  try {
    return readFileSync(join(SQL_DIR, filename), 'utf-8');
  } catch (error) {
    throw new Error(`Failed to load SQL file ${filename}: ${error}`);
  }
};

export const SCHEMA_SQL = loadSQL('schema.sql');
