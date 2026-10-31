import {fileURLToPath} from 'node:url';
const directory=fileURLToPath(new URL('./🗑️generated/', import.meta.url));
for(const key of ['SEMIO_TEST_ARTIFACT_ROOT','TEMP','TMP','TMPDIR']) process.env[key]=directory;
