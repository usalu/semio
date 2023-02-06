using System;
using System.Security.Cryptography;
using System.Text;
using Google.Protobuf;

namespace Semio
{
    public static class Utils
    {
        public static byte[] ToBytes(dynamic semioObject) => ((IMessage)semioObject).ToByteArray();

        public static string ToByteString(dynamic semioObject) => ByteString.CopyFrom(ToBytes(semioObject)).ToStringUtf8();
        public static string ToBase64(dynamic semioObject) => Convert.ToBase64String(ToBytes(semioObject));
        public static string ToMd5Hash(dynamic semioObject)
        {
            string hash;
            using (MD5 md5 = MD5.Create())
            {
                byte[] hashBytes = md5.ComputeHash(Utils.ToBytes(semioObject));
                StringBuilder sb = new System.Text.StringBuilder();
                for (int i = 0; i < hashBytes.Length; i++)
                    sb.Append(hashBytes[i].ToString("X2"));
                hash = sb.ToString();
            }
            return hash;
        }

       
}
}
