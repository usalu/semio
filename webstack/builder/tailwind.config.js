export default {
    content: [
      "./src/renderer/src/index.html",
      "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
      colors: {
        'primary': '#ff344f',
        'secondary': '#34d1bf',
        'tertiary': '#fa9500',
        'dark': '#002430',
        'light': '#f7f3e3',
        'danger': '#a60009',
        'success': '#7eb77f',
        'warning': '#fccf05',
        'info': '#dbbea1',
      },
      fontFamily: {
        sans: ['Anta', 'sans-serif'],
      },
      extend: {
        fontFamily: {
          'anta': ['Anta', 'Verdana', 'sans-serif']
        }
      },
    },
    plugins: [],
  }