// import { Config } from 'postcss-load-config';
// import { postcssConfig } from "@semio/core";

// const config: Config = {
//   ...postcssConfig,
// };

// export default config;

import { Config } from 'postcss-load-config';

const config: Config = {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};

export default config;