export default {
    content: ['./src/renderer/src/index.html', './src/**/*.{js,ts,jsx,tsx}'],
    theme: {
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
        fontFamily: {
            sans: ['Anta', 'Noto Emoji', 'sans-serif']
        },
        extend: {
            fontFamily: {
                noto: ['Noto Emoji', 'Anta', 'sans-serif'],
                anta: ['Anta', 'Verdana', 'sans-serif']
            },
            borderWidth: {
                'thin': '0.5px',
              },
        }
    },
    plugins: []
}
