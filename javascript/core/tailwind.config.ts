import type { Config } from "tailwindcss";

export default {
  content: ["./**/*.{ts,tsx,mdx}"],
  theme: {
    extend: {
      colors: {
        primary: '#ff344f',
        secondary: '#34d1bf',
        tertiary: '#fa9500',
        dark: '#001117',
        darkGrey: '#001821',
        grey: '#29383c',
        lightGrey: '#cecbbd',
        light: '#f7f3e3',
        danger: '#a60009',
        warning: '#fccf05',
        info: '#dbbea1',
        success: '#7eb77f',
      },
      borderWidth: {
        'thin': '0.5px',
      },
    },
  },
  plugins: [],
} satisfies Config;
